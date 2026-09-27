const form = document.getElementById('connect-form');
const address = document.getElementById('address');
const button = document.getElementById('connect');
const error = document.getElementById('error');

form.addEventListener('submit', async (event) => {
  event.preventDefault();
  if (button.disabled) return;
  error.textContent = '';
  button.disabled = true;
  button.textContent = '正在打开…';
  try {
    await window.__TAURI__.core.invoke('connect_harness', { address: address.value.trim() });
    // Do not persist authentication tokens or leave the previous token in the form.
    address.value = '';
  } catch (message) {
    error.textContent = typeof message === 'string' ? message : '连接失败，请检查地址后重试。';
    address.focus();
  } finally {
    button.disabled = false;
    button.textContent = '连接 Harness';
  }
});
