// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="index.html"><strong aria-hidden="true">1.</strong> The Rustyscript Book</a></li><li class="chapter-item expanded affix "><li class="spacer"></li><li class="chapter-item expanded "><a href="getting_started/index.html"><strong aria-hidden="true">2.</strong> Getting Started</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="getting_started/hello_world.html"><strong aria-hidden="true">2.1.</strong> Hello World</a></li><li class="chapter-item expanded "><a href="getting_started/calling_functions.html"><strong aria-hidden="true">2.2.</strong> Calling Functions</a></li><li class="chapter-item expanded "><a href="getting_started/errors.html"><strong aria-hidden="true">2.3.</strong> Error Handling</a></li><li class="chapter-item expanded "><a href="getting_started/using_javascript_types_in_rust.html"><strong aria-hidden="true">2.4.</strong> Using JavaScript Types in Rust</a></li><li class="chapter-item expanded "><a href="getting_started/modules.html"><strong aria-hidden="true">2.5.</strong> On Modules and import</a></li><li class="chapter-item expanded "><a href="getting_started/sandbox.html"><strong aria-hidden="true">2.6.</strong> The Sandbox</a></li><li class="chapter-item expanded "><a href="getting_started/runtime_options.html"><strong aria-hidden="true">2.7.</strong> Runtime Options</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="getting_started/extension_options.html"><strong aria-hidden="true">2.7.1.</strong> Extension Options</a></li></ol></li></ol></li><li class="chapter-item expanded "><li class="spacer"></li><li class="chapter-item expanded "><a href="advanced/index.html"><strong aria-hidden="true">3.</strong> Advanced Topics</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="advanced/asynchronous_javascript.html"><strong aria-hidden="true">3.1.</strong> Asynchronous JavaScript</a></li><li class="chapter-item expanded "><a href="advanced/calling_rust_from_javascript.html"><strong aria-hidden="true">3.2.</strong> Calling Rust from JavaScript</a></li><li class="chapter-item expanded "><a href="advanced/custom_extensions.html"><strong aria-hidden="true">3.3.</strong> Custom Extensions</a></li><li class="chapter-item expanded "><a href="advanced/static_runtime.html"><strong aria-hidden="true">3.4.</strong> Static Runtimes</a></li><li class="chapter-item expanded "><a href="advanced/multithreading.html"><strong aria-hidden="true">3.5.</strong> Multi-Threading</a></li><li class="chapter-item expanded "><a href="advanced/nodejs_compatibility.html"><strong aria-hidden="true">3.6.</strong> NodeJS Compatibility</a></li><li class="chapter-item expanded "><a href="advanced/permissions.html"><strong aria-hidden="true">3.7.</strong> Permissions</a></li><li class="chapter-item expanded "><a href="advanced/snapshots.html"><strong aria-hidden="true">3.8.</strong> Snapshots</a></li></ol></li><li class="chapter-item expanded "><li class="spacer"></li><li class="chapter-item expanded "><a href="extensions/index.html"><strong aria-hidden="true">4.</strong> Extensions</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="extensions/safe_extensions.html"><strong aria-hidden="true">4.1.</strong> Safe Extensions</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="extensions/console.html"><strong aria-hidden="true">4.1.1.</strong> Console</a></li><li class="chapter-item expanded "><a href="extensions/crypto.html"><strong aria-hidden="true">4.1.2.</strong> Crypto</a></li><li class="chapter-item expanded "><a href="extensions/url.html"><strong aria-hidden="true">4.1.3.</strong> Url</a></li><li class="chapter-item expanded "><a href="extensions/web_stub.html"><strong aria-hidden="true">4.1.4.</strong> Web Stub</a></li></ol></li><li class="chapter-item expanded "><a href="extensions/io_extensions.html"><strong aria-hidden="true">4.2.</strong> IO Extensions</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="extensions/cache.html"><strong aria-hidden="true">4.2.1.</strong> Cache</a></li><li class="chapter-item expanded "><a href="extensions/cron.html"><strong aria-hidden="true">4.2.2.</strong> Cron</a></li><li class="chapter-item expanded "><a href="extensions/ffi.html"><strong aria-hidden="true">4.2.3.</strong> FFI</a></li><li class="chapter-item expanded "><a href="extensions/fs.html"><strong aria-hidden="true">4.2.4.</strong> FS</a></li><li class="chapter-item expanded "><a href="extensions/io.html"><strong aria-hidden="true">4.2.5.</strong> IO</a></li><li class="chapter-item expanded "><a href="extensions/kv.html"><strong aria-hidden="true">4.2.6.</strong> KV</a></li><li class="chapter-item expanded "><a href="extensions/webgpu.html"><strong aria-hidden="true">4.2.7.</strong> WebGPU</a></li></ol></li><li class="chapter-item expanded "><a href="extensions/network_extensions.html"><strong aria-hidden="true">4.3.</strong> Network Extensions</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="extensions/broadcast_channel.html"><strong aria-hidden="true">4.3.1.</strong> Broadcast Channel</a></li><li class="chapter-item expanded "><a href="extensions/http.html"><strong aria-hidden="true">4.3.2.</strong> HTTP</a></li><li class="chapter-item expanded "><a href="extensions/web.html"><strong aria-hidden="true">4.3.3.</strong> Web</a></li><li class="chapter-item expanded "><a href="extensions/websocket.html"><strong aria-hidden="true">4.3.4.</strong> WebSocket</a></li><li class="chapter-item expanded "><a href="extensions/webstorage.html"><strong aria-hidden="true">4.3.5.</strong> WebStorage</a></li></ol></li><li class="chapter-item expanded "><a href="extensions/nodejs_extensions.html"><strong aria-hidden="true">4.4.</strong> NodeJS Extensions</a></li></ol></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString();
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
