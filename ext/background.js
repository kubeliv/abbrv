console.log("Using as baseUrl: " + baseUrl);

chrome.webRequest.onBeforeRequest.addListener(function (details) {
  let url = new URL(details.url);
  let path = url.toString().split(url.host)[1];
  return {redirectUrl: baseUrl + path};
}, {
  urls: ['*://go/*']
}, ['blocking']);
