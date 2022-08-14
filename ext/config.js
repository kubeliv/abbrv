let baseUrl = "";

chrome.storage.sync.get({
  baseUrl: "https://example.com",
}, function (items) {
  baseUrl = items.baseUrl;
  console.log("Using baseUrl " + baseUrl);
});

chrome.storage.onChanged.addListener(function(changes, area) {
  if ("baseUrl" in changes) {
    baseUrl = changes["baseUrl"].newValue;
    console.log("Using baseUrl " + baseUrl);
  }
});
