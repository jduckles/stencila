use codec::{
    common::futures::executor::block_on,
    schema::{shortcuts::*, ClaimType, NoteType},
    EncodeOptions,
};

/// Encode a node to a string using a codec with options
fn to_string(codec: &str, options: Option<EncodeOptions>) {
    // The aim is to have one of each content node type
    let node = art([
        // Heading
        h1([t("heading")]),
        // Paragraph
        p([
            // Text
            t("text"),
            // Marks
            em([t("emphasis")]),
            q([t("q")]),
            stg([t("strong")]),
            stk([t("strike")]),
            sub([t("subscript")]),
            sup([t("superscript")]),
            u([t("underline")]),
            // Media objects
            aud("url"),
            img("url"),
            vid("url"),
            // Other
            btn("name", "code"),
            ce("code", Some("lang")),
            cf("code"),
            ct("target"),
            ctg(["target1", "target2"]),
            del([t("text")]),
            ins([t("text")]),
            lnk([t("link")], "url"),
            mf("code", Some("lang")),
            nte(NoteType::Endnote, [p([t("text")])]),
            par("name"),
            spn("code", [t("span")]),
        ]),
        // Quote, math and code blocks
        cb("code block", Some("lang")),
        cc("code chunk", Some("lang")),
        mb("math block", Some("lang")),
        qb([p([t("quote block")])]),
        // List
        ol([li([t("text")])]),
        ul([li([t("text")])]),
        // Table
        tab([tr([th([t("col")])]), tr([td([t("cell")])])]),
        // Other
        cal("source", [arg("name", "code")]),
        clm(ClaimType::Lemma, [p([t("text")])]),
        div("code", [p([t("text")])]),
        fig([p([img("url")])]),
        r#if([ifc("code", Some("lang"), [p([t("text")])])]),
        inc("source"),
        r#for("symbol", "code", [p([t("text")])]),
        sec([p([t("text")])]),
        tb(),
    ]);

    let codec = codecs::get(Some(&String::from(codec)), None, None).expect("Should find codec");

    block_on(async move {
        // To minimize the proportion of time spent on spawning async task, constructing the node,
        // getting codec etc, this performs multiple iterations of encoding.
        for _iter in 0..100 {
            codec
                .to_string(&node, options.clone())
                .await
                .expect("Should encode successfully");
        }
    })
}

pub fn main() {
    divan::main();
}

#[divan::bench]
fn html() {
    to_string("html", None)
}

#[divan::bench]
fn html_compact() {
    to_string(
        "html",
        Some(EncodeOptions {
            compact: true,
            ..Default::default()
        }),
    )
}

#[divan::bench]
fn jats() {
    to_string("jats", None)
}

#[divan::bench]
fn jats_compact() {
    to_string(
        "jats",
        Some(EncodeOptions {
            compact: true,
            ..Default::default()
        }),
    )
}

#[divan::bench]
fn json() {
    to_string("json", None)
}

#[divan::bench]
fn json_compact() {
    to_string(
        "json",
        Some(EncodeOptions {
            compact: true,
            ..Default::default()
        }),
    )
}

#[divan::bench]
fn json5() {
    to_string("json5", None)
}

#[divan::bench]
fn json5_compact() {
    to_string(
        "json5",
        Some(EncodeOptions {
            compact: true,
            ..Default::default()
        }),
    )
}

#[divan::bench]
fn markdown() {
    to_string("markdown", None)
}

#[divan::bench]
fn text() {
    to_string("text", None)
}

#[divan::bench]
fn yaml() {
    to_string("yaml", None)
}
