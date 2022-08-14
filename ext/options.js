// Saves options to chrome.storage
function save_options() {
  let baseUrl = document.getElementById('base_url').value;
  chrome.storage.sync.set({
    baseUrl: baseUrl,
  }, function () {
    // Update status to let user know options were saved.
    let status = document.getElementById('status');
    status.textContent = 'Options saved.';
    setTimeout(function () {
      status.textContent = '';
    }, 2500);
  });
}

// Restores state using the preferences stored in chrome.storage.
function restore_options() {
  chrome.storage.sync.get({
    baseUrl: "https://example.com",
  }, function (items) {
    document.getElementById('base_url').value = items.baseUrl;
  });
}

document.addEventListener('DOMContentLoaded', restore_options);
document.getElementById('save').addEventListener('click', save_options);
