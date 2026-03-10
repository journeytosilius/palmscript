function wirePalmScriptHeaderHomeLink() {
  const topic = document.querySelector(".md-header__title .md-header__topic");
  if (!topic || topic.querySelector(".ps-home-link")) {
    return;
  }

  const label = topic.textContent.trim();
  if (!label) {
    return;
  }

  const link = document.createElement("a");
  link.href = "https://palmscript.dev/";
  link.className = "ps-home-link";
  link.setAttribute("aria-label", "PalmScript home");
  link.textContent = label;

  topic.replaceChildren(link);
}

if (typeof document$ !== "undefined") {
  document$.subscribe(wirePalmScriptHeaderHomeLink);
} else {
  document.addEventListener("DOMContentLoaded", wirePalmScriptHeaderHomeLink);
}
