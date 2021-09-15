window.addEventListener('DOMContentLoaded', () => {
  const bm = require('big_mac');

  const replaceText = (selector, text) => {
    const element = document.getElementById(selector)
    if (element) element.innerText = text
  }

  replaceText(`mac-address`, bm.get_mac());
})
