/**
 * Explog Theme JavaScript
 * - Code block copy button (Prism handles syntax highlighting)
 */

document.addEventListener('DOMContentLoaded', function () {
    initCodeBlocks();
});

/**
 * Initialize code blocks with copy buttons
 */
function initCodeBlocks() {
    const codeBlocks = document.querySelectorAll('.post-content pre, .page-content pre');

    codeBlocks.forEach(function (pre) {
        // Wrap in container if not already wrapped
        if (!pre.parentElement.classList.contains('code-block')) {
            const wrapper = document.createElement('div');
            wrapper.className = 'code-block';
            pre.parentElement.insertBefore(wrapper, pre);
            wrapper.appendChild(pre);
        }

        // Add copy button
        const copyBtn = document.createElement('button');
        copyBtn.className = 'copy-btn';
        copyBtn.textContent = 'Copy';
        copyBtn.setAttribute('aria-label', 'Copy code to clipboard');

        copyBtn.addEventListener('click', function () {
            const code = pre.querySelector('code') || pre;
            const text = code.textContent;

            navigator.clipboard.writeText(text).then(function () {
                copyBtn.textContent = 'Copied!';
                copyBtn.classList.add('copied');

                setTimeout(function () {
                    copyBtn.textContent = 'Copy';
                    copyBtn.classList.remove('copied');
                }, 2000);
            }).catch(function (err) {
                console.error('Failed to copy:', err);
                copyBtn.textContent = 'Error';
                setTimeout(function () {
                    copyBtn.textContent = 'Copy';
                }, 2000);
            });
        });

        pre.parentElement.insertBefore(copyBtn, pre);
    });
}
