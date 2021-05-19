import { Component, h } from '@stencil/core'
import { Route } from '@stencil/router'
import { SettingsRouter } from '../settingsRouter'

@Component({
  tag: 'app-settings-root',
  styleUrl: 'app-settings-root.css',
  scoped: true,
})
export class AppSettings {
  render() {
    return (
      <div class="settingsWindow">
        <app-settings-sidebar></app-settings-sidebar>

        <main>
          <SettingsRouter.Switch>
            <Route path="/settings" to="/settings/advanced" />

            <Route path="/settings/advanced">
              <app-settings-advanced></app-settings-advanced>
            </Route>

            <Route path="/settings/plugins">
              <app-settings-plugins></app-settings-plugins>
            </Route>
          </SettingsRouter.Switch>
        </main>
      </div>
    )
  }
}
