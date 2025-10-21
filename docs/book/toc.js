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
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="index.html">Introduction</a></li><li class="chapter-item expanded affix "><li class="part-title">Getting Started</li><li class="chapter-item expanded "><a href="tutorials/getting-started.html"><strong aria-hidden="true">1.</strong> Quick Start</a></li><li class="chapter-item expanded "><a href="tutorials/installation.html"><strong aria-hidden="true">2.</strong> Installation</a></li><li class="chapter-item expanded "><a href="tutorials/first-scan.html"><strong aria-hidden="true">3.</strong> First Scan</a></li><li class="chapter-item expanded affix "><li class="part-title">User Guide</li><li class="chapter-item expanded "><a href="tutorials/basic-usage.html"><strong aria-hidden="true">4.</strong> Basic Usage</a></li><li class="chapter-item expanded "><a href="tutorials/advanced-usage.html"><strong aria-hidden="true">5.</strong> Advanced Features</a></li><li class="chapter-item expanded "><a href="configuration/overview.html"><strong aria-hidden="true">6.</strong> Configuration</a></li><li class="chapter-item expanded "><a href="tutorials/custom-detectors.html"><strong aria-hidden="true">7.</strong> Custom Detectors</a></li><li class="chapter-item expanded "><a href="tutorials/automation.html"><strong aria-hidden="true">8.</strong> Automation</a></li><li class="chapter-item expanded affix "><li class="part-title">API Reference</li><li class="chapter-item expanded "><a href="api/core.html"><strong aria-hidden="true">9.</strong> Core Library API</a></li><li class="chapter-item expanded "><a href="api/cli.html"><strong aria-hidden="true">10.</strong> CLI Commands</a></li><li class="chapter-item expanded "><a href="configuration/schema.html"><strong aria-hidden="true">11.</strong> Configuration Schema</a></li><li class="chapter-item expanded "><a href="api/detectors.html"><strong aria-hidden="true">12.</strong> Detector API</a></li><li class="chapter-item expanded affix "><li class="part-title">Architecture</li><li class="chapter-item expanded "><a href="architecture/overview.html"><strong aria-hidden="true">13.</strong> Overview</a></li><li class="chapter-item expanded "><a href="architecture/decisions/index.html"><strong aria-hidden="true">14.</strong> Architecture Decisions</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="architecture/decisions/ADR-001-modular-crate-structure.html"><strong aria-hidden="true">14.1.</strong> ADR-001: Modular Crate Structure</a></li><li class="chapter-item expanded "><a href="architecture/decisions/ADR-002-performance-optimization.html"><strong aria-hidden="true">14.2.</strong> ADR-002: Performance Optimization Strategy</a></li><li class="chapter-item expanded "><a href="architecture/decisions/ADR-003-security-detection.html"><strong aria-hidden="true">14.3.</strong> ADR-003: Security Detection Framework</a></li><li class="chapter-item expanded "><a href="architecture/decisions/ADR-004-llm-integration.html"><strong aria-hidden="true">14.4.</strong> ADR-004: LLM Integration</a></li></ol></li><li class="chapter-item expanded "><a href="performance/overview.html"><strong aria-hidden="true">15.</strong> Performance</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="performance/latest.html"><strong aria-hidden="true">15.1.</strong> Latest Benchmarks</a></li><li class="chapter-item expanded "><a href="performance/optimization.html"><strong aria-hidden="true">15.2.</strong> Optimization Guide</a></li></ol></li><li class="chapter-item expanded "><li class="part-title">Integrations</li><li class="chapter-item expanded "><a href="integrations/ci-cd.html"><strong aria-hidden="true">16.</strong> CI/CD Integration</a></li><li class="chapter-item expanded "><a href="integrations/git.html"><strong aria-hidden="true">17.</strong> Git Integration</a></li><li class="chapter-item expanded "><a href="integrations/docker.html"><strong aria-hidden="true">18.</strong> Docker Integration</a></li><li class="chapter-item expanded "><a href="integrations/kubernetes.html"><strong aria-hidden="true">19.</strong> Kubernetes Integration</a></li><li class="chapter-item expanded affix "><li class="part-title">Examples</li><li class="chapter-item expanded "><a href="examples/cli.html"><strong aria-hidden="true">20.</strong> CLI Examples</a></li><li class="chapter-item expanded "><a href="examples/library.html"><strong aria-hidden="true">21.</strong> Library Usage</a></li><li class="chapter-item expanded "><a href="examples/custom-detectors.html"><strong aria-hidden="true">22.</strong> Custom Detectors</a></li><li class="chapter-item expanded "><a href="examples/production.html"><strong aria-hidden="true">23.</strong> Production Setup</a></li><li class="chapter-item expanded affix "><li class="part-title">Contributing</li><li class="chapter-item expanded "><a href="contributing/development.html"><strong aria-hidden="true">24.</strong> Development Setup</a></li><li class="chapter-item expanded "><a href="contributing/code-style.html"><strong aria-hidden="true">25.</strong> Code Style</a></li><li class="chapter-item expanded "><a href="contributing/testing.html"><strong aria-hidden="true">26.</strong> Testing</a></li><li class="chapter-item expanded "><a href="contributing/documentation.html"><strong aria-hidden="true">27.</strong> Documentation</a></li><li class="chapter-item expanded affix "><li class="part-title">Reference</li><li class="chapter-item expanded "><a href="CHANGELOG.html"><strong aria-hidden="true">28.</strong> Changelog</a></li><li class="chapter-item expanded "><a href="LICENSE.html"><strong aria-hidden="true">29.</strong> License</a></li><li class="chapter-item expanded "><a href="SECURITY.html"><strong aria-hidden="true">30.</strong> Security</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
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
