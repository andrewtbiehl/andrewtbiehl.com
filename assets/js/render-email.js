---
---

(function () {

"use strict";

function renderEmail(localPart, ignored1, hostName, ignored2, topLevelDomain) {
  const email = localPart + "@" + hostName + "." + topLevelDomain;
  const emailPrefix = "ma" + "il" + "to";
  const emailLink = document.getElementById("email-link-id");
  emailLink.href = emailPrefix + ":" + email;
  emailLink.textContent = email;
}

renderEmail(
  "{{ site.email.local_part }}",
  "this is a red herring",
  "{{ site.email.host_name }}",
  "this is also a red herring",
  "{{ site.email.top_level_domain }}"
);

})();
