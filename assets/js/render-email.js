---
---

/**
Renders my email address so that it is available on the website but is more difficult
to scrape. Note that some code obfuscation is utilized.
*/

(function () {

"use strict";

function renderEmail(localPart, ignored1, hostName, ignored2, topLevelDomain) {
  const email = localPart + "@" + hostName + "." + topLevelDomain;
  const emailPrefix = "ma" + "il" + "to";
  const emailLink = document.createElement("a");
  emailLink.href = emailPrefix + ":" + email;
  emailLink.textContent = email;
  document.getElementById("email-info-id").replaceWith(emailLink);
}

renderEmail(
  "{{ site.email.local_part }}",
  "this is a red herring",
  "{{ site.email.host_name }}",
  "this is also a red herring",
  "{{ site.email.top_level_domain }}"
);

})();
