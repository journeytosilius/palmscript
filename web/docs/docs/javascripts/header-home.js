const DEFAULT_PALMSCRIPT_LOCALE = "en";
const PALMSCRIPT_HOME_LOCALE_PATHS = {
  en: "",
  es: "es",
  "pt-BR": "pt-br",
  de: "de",
  ja: "ja",
  fr: "fr",
};

function normalizePalmScriptLocaleCode(locale) {
  if (!locale) {
    return null;
  }

  const trimmedLocale = locale.trim();
  if (!trimmedLocale) {
    return null;
  }

  for (const supportedLocale of Object.keys(PALMSCRIPT_HOME_LOCALE_PATHS)) {
    if (supportedLocale.toLowerCase() === trimmedLocale.toLowerCase()) {
      return supportedLocale;
    }
  }

  const primaryLanguage = trimmedLocale.split("-")[0]?.toLowerCase();
  if (!primaryLanguage) {
    return null;
  }

  for (const supportedLocale of Object.keys(PALMSCRIPT_HOME_LOCALE_PATHS)) {
    if (supportedLocale.toLowerCase() === primaryLanguage) {
      return supportedLocale;
    }
  }

  return null;
}

function getPalmScriptDocsLocaleCode(locationLike) {
  const activeLocation = locationLike ?? window.location;
  const pathnameLocale = normalizePalmScriptLocaleCode(
    activeLocation.pathname.match(/^\/([^/]+)\/docs(?:\/|$)/)?.[1],
  );
  if (pathnameLocale) {
    return pathnameLocale;
  }

  const documentLocale =
    typeof document !== "undefined"
      ? normalizePalmScriptLocaleCode(document.documentElement?.lang)
      : null;
  if (documentLocale) {
    return documentLocale;
  }

  return DEFAULT_PALMSCRIPT_LOCALE;
}

function getPalmScriptLocaleHomeHref(locationLike) {
  const activeLocation = locationLike ?? window.location;
  const locale = getPalmScriptDocsLocaleCode(activeLocation);
  const homeLocalePath = PALMSCRIPT_HOME_LOCALE_PATHS[locale] ?? "";
  if (!homeLocalePath) {
    return `${activeLocation.origin}/`;
  }

  return `${activeLocation.origin}/${homeLocalePath}/`;
}

function applyPalmScriptHomeLinkAttributes(link, href, label) {
  link.href = href;
  link.className = "ps-home-link";
  link.setAttribute("aria-label", "PalmScript home");
  link.textContent = label;
  link.removeAttribute("target");
  link.removeAttribute("rel");
  return link;
}

function buildPalmScriptHomeLink(label, href = getPalmScriptLocaleHomeHref()) {
  const link = document.createElement("a");
  return applyPalmScriptHomeLinkAttributes(link, href, label);
}

function upsertPalmScriptHomeLink(topic, label) {
  const href = getPalmScriptLocaleHomeHref();
  const existingLink = topic.querySelector(".ps-home-link");
  if (existingLink) {
    applyPalmScriptHomeLinkAttributes(existingLink, href, label);
    return;
  }

  topic.replaceChildren(buildPalmScriptHomeLink(label, href));
}

function wirePalmScriptHeaderHomeLink() {
  const topic = document.querySelector(".md-header__title .md-header__topic");
  if (!topic) {
    return;
  }

  upsertPalmScriptHomeLink(topic, "PalmScript");
}

function wirePalmScriptHeaderTopicHomeLink() {
  const topic = document.querySelector(
    '.md-header__title [data-md-component="header-topic"] .md-ellipsis',
  );
  if (!topic) {
    return;
  }

  const label = topic.textContent?.trim();
  if (!label) {
    return;
  }

  upsertPalmScriptHomeLink(topic, label);
}

function wirePalmScriptHeaderLogoLink() {
  const href = getPalmScriptLocaleHomeHref();
  const logos = document.querySelectorAll("a.md-logo");
  for (const logo of logos) {
    logo.href = href;
    logo.removeAttribute("target");
    logo.removeAttribute("rel");
  }
}

function isExternalHref(href) {
  if (!href || href.startsWith("#")) {
    return false;
  }

  if (href.startsWith("mailto:") || href.startsWith("tel:")) {
    return true;
  }

  try {
    const url = new URL(href, window.location.href);
    return url.origin !== window.location.origin;
  } catch {
    return false;
  }
}

function wirePalmScriptDocsLinks() {
  const links = document.querySelectorAll("a[href]");
  for (const link of links) {
    const href = link.getAttribute("href");
    if (!href || href.startsWith("#")) {
      continue;
    }

    if (isExternalHref(href)) {
      link.setAttribute("target", "_blank");
      link.setAttribute("rel", "noopener noreferrer");
      continue;
    }

    link.removeAttribute("target");
    link.removeAttribute("rel");
  }
}

function wirePalmScriptDocsUi() {
  wirePalmScriptHeaderLogoLink();
  wirePalmScriptHeaderHomeLink();
  wirePalmScriptHeaderTopicHomeLink();
  wirePalmScriptDocsLinks();
}

if (typeof document !== "undefined") {
  if (typeof document$ !== "undefined") {
    document$.subscribe(wirePalmScriptDocsUi);
  } else {
    document.addEventListener("DOMContentLoaded", wirePalmScriptDocsUi);
  }
}

if (typeof module !== "undefined" && module.exports) {
  module.exports = {
    applyPalmScriptHomeLinkAttributes,
    getPalmScriptDocsLocaleCode,
    getPalmScriptLocaleHomeHref,
    normalizePalmScriptLocaleCode,
  };
}
