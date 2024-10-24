import * as vscode from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node";

import { registerAuthenticationProvider } from "./authentication";
import { registerDocumentCommands } from "./commands";
import { registerNotifications } from "./notifications";
import { collectSecrets, registerSecretsCommands } from "./secrets";
import { registerStatusBar } from "./status-bar";
import { closeDocumentViewPanels } from "./webviews";
import { cliPath } from "./clis";
import { registerWalkthroughCommands } from "./walkthroughs";

let client: LanguageClient | undefined;
let outputChannel = vscode.window.createOutputChannel(
  "Stencila Language Server"
);

/**
 * Activate the extension
 */
export async function activate(context: vscode.ExtensionContext) {
  // Register auth provider, commands etc
  // Some of these (e.g. auth provider) are used when collecting secrets in `startServer`
  // so this needs to be done first
  registerAuthenticationProvider(context);
  registerSecretsCommands(context);
  registerDocumentCommands(context);
  registerStatusBar(context);
  registerWalkthroughCommands(context);
  registerOtherCommands(context);

  await startServer(context);
}

/**
 * Start the Stencila LSP server
 */
async function startServer(context: vscode.ExtensionContext) {
  // Get config options
  const initializationOptions = vscode.workspace.getConfiguration("stencila");

  // Get the path to the CLI
  let command = cliPath(context);

  // Determine the arguments to the CLI
  let args: string[];
  const logLevel = initializationOptions.languageServer?.logLevel;
  switch (context.extensionMode) {
    case vscode.ExtensionMode.Development:
    case vscode.ExtensionMode.Test: {
      args = ["lsp", `--log-level=${logLevel ?? "debug"}`];
      break;
    }
    case vscode.ExtensionMode.Production: {
      args = ["lsp", `--log-level=${logLevel ?? "warn"}`];
      break;
    }
  }

  // Collect secrets to pass as env vars to LSP server
  const secrets = await collectSecrets(context);

  // Start the language server client passing secrets as env vars
  const serverOptions: ServerOptions = {
    command,
    args,
    options: { env: { ...process.env, ...secrets } },
  };
  const clientOptions: LanguageClientOptions = {
    initializationOptions,
    outputChannel,
    documentSelector: [{ language: "smd" }, { language: "myst" }],
    markdown: {
      isTrusted: true,
      supportHtml: true,
    },
  };
  client = new LanguageClient(
    "stencila",
    "Stencila",
    serverOptions,
    clientOptions
  );
  await client.start();

  // Register notifications on the client
  registerNotifications(client);
}

/**
 * Register other commands
 */
function registerOtherCommands(context: vscode.ExtensionContext) {
  // Command to open stencila settings
  context.subscriptions.push(
    vscode.commands.registerCommand("stencila.settings", () => {
      vscode.commands.executeCommand(
        "workbench.action.openSettings",
        "stencila"
      );
    })
  );

  // Command to restart the server (e.g. after setting or removing secrets)
  context.subscriptions.push(
    vscode.commands.registerCommand("stencila.lsp-server.restart", async () => {
      if (client) {
        try {
          await client.stop();
        } catch (error) {
          vscode.window.showWarningMessage(
            `Error stopping the Stencila Language Server: ${error}. Proceeding with restart.`
          );
        } finally {
          client = undefined;
        }
      }

      // Close all doc view panels which will otherwise be left unresponsive
      closeDocumentViewPanels();

      // Wait a bit before starting the new server
      await new Promise((resolve) => setTimeout(resolve, 1000));

      await startServer(context);

      vscode.window.showInformationMessage(
        "Stencila Language Server has been restarted."
      );
    })
  );

  // Command to view the server logs
  context.subscriptions.push(
    vscode.commands.registerCommand("stencila.lsp-server.logs", async () => {
      outputChannel.show();
    })
  );
}

/**
 * Deactivate the extension
 */
export function deactivate() {
  if (client) {
    client.stop();
  }
}

/**
 * Subscribe to content of a document in a specific format
 */
export async function subscribeToContent(
  documentUri: vscode.Uri,
  format: string,
  callback: (content: string) => void
): Promise<string> {
  if (!client) {
    throw new Error("No Stencila LSP client");
  }

  const content = (await client.sendRequest("stencila/subscribeContent", {
    uri: documentUri.toString(),
    format,
  })) as string;

  callback(content);

  client.onNotification(
    "stencila/publishContent",
    (published: { uri: string; format: string; content: string }) => {
      if (
        published.uri === documentUri.toString() &&
        published.format === format
      ) {
        callback(published.content);
      }
    }
  );

  return content;
}
