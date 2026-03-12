const assert = require("node:assert/strict");
const test = require("node:test");

const {
  applyPalmScriptHomeLinkAttributes,
  getPalmScriptLocaleHomeHref,
  normalizePalmScriptLocaleCode,
} = require("../docs/javascripts/header-home.js");

function createFakeLink() {
  return {
    href: "https://palmscript.dev/",
    className: "",
    textContent: "",
    attributes: new Map([
      ["target", "_blank"],
      ["rel", "noopener noreferrer"],
    ]),
    setAttribute(name, value) {
      this.attributes.set(name, value);
    },
    removeAttribute(name) {
      this.attributes.delete(name);
    },
    getAttribute(name) {
      return this.attributes.get(name);
    },
  };
}

test("normalizePalmScriptLocaleCode resolves supported locale variants", () => {
  assert.equal(normalizePalmScriptLocaleCode("es"), "es");
  assert.equal(normalizePalmScriptLocaleCode("ES"), "es");
  assert.equal(normalizePalmScriptLocaleCode("pt-br"), "pt-BR");
  assert.equal(normalizePalmScriptLocaleCode("fr-CA"), "fr");
  assert.equal(normalizePalmScriptLocaleCode(""), null);
});

test("getPalmScriptLocaleHomeHref maps docs locales to homepage locales", () => {
  assert.equal(
    getPalmScriptLocaleHomeHref({
      origin: "https://palmscript.dev",
      pathname: "/docs/",
    }),
    "https://palmscript.dev/",
  );
  assert.equal(
    getPalmScriptLocaleHomeHref({
      origin: "https://palmscript.dev",
      pathname: "/es/docs/learn/installation/",
    }),
    "https://palmscript.dev/es/",
  );
  assert.equal(
    getPalmScriptLocaleHomeHref({
      origin: "https://palmscript.dev",
      pathname: "/pt-BR/docs/",
    }),
    "https://palmscript.dev/pt-br/",
  );
});

test("applyPalmScriptHomeLinkAttributes refreshes existing localized links", () => {
  const link = createFakeLink();

  applyPalmScriptHomeLinkAttributes(
    link,
    "https://palmscript.dev/es/",
    "PalmScript",
  );

  assert.equal(link.href, "https://palmscript.dev/es/");
  assert.equal(link.className, "ps-home-link");
  assert.equal(link.textContent, "PalmScript");
  assert.equal(link.getAttribute("aria-label"), "PalmScript home");
  assert.equal(link.getAttribute("target"), undefined);
  assert.equal(link.getAttribute("rel"), undefined);
});
