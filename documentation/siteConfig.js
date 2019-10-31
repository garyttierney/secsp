const siteConfig = {
  title: 'SELinux C-style policy', // Title for your website.
  tagline: 'Modern policy language for SELinux',
  url: 'https://secsp.netlify.com', // Your website URL
  baseUrl: '/', // Base URL for your project */
  // For github.io type URLs, you would set the url and baseUrl like:
  //   url: 'https://facebook.github.io',
  //   baseUrl: '/test-site/',

  // Used for publishing and more
  projectName: 'secsp',
  organizationName: 'garyttierney',
  // For top-level user or org sites, the organization is still the same.
  // e.g., for the https://JoelMarcey.github.io site, it would be set like...
  //   organizationName: 'JoelMarcey'

  // For no header links in the top nav bar -> headerLinks: [],
  headerLinks: [
    { doc: 'user', label: 'User Guide' },
    { doc: 'reference', label: 'Syntax Reference' },
    { doc: 'developer', label: 'Developer Documentation' },
    { page: 'help', label: 'Help' },
  ],
  customDocsPath: './documentation/docs/',

  /* Colors for website */
  colors: {
    secondaryColor: '#383749',
    primaryColor: '#2a2936',
  },

  /* Custom fonts for website */
  /*
  fonts: {
    myFont: [
      "Times New Roman",
      "Serif"
    ],
    myOtherFont: [
      "-apple-system",
      "system-ui"
    ]
  },
  */

  // This copyright info is used in /core/Footer.js and blog RSS/Atom feeds.
  copyright: `Copyright © ${new Date().getFullYear()} Gary Tierney`,

  highlight: {
    // Highlight.js theme to use for syntax highlighting in code blocks.
    theme: 'default',
  },

  // On page navigation for the current documentation page.
  onPageNav: 'separate',
  // No .html extensions for paths.
  cleanUrl: false,

  // For sites with a sizable amount of content, set collapsible to true.
  // Expand/collapse the links and subcategories under categories.
  docsSideNavCollapsible: false,

  // Show documentation's last contributor's name.
  enableUpdateBy: true,

  // Show documentation's last update time.
  enableUpdateTime: true,

  scripts: [
    '/js/editor.worker.js',
    '/js/vendors.js',
    '/js/monaco.js',
    '/js/csp-wasm-editor.js',
  ]
  // You may provide arbitrary config keys to be used as needed by your
  // template. For example, if you need your repo's URL...
  //   repoUrl: 'https://github.com/facebook/test-site',
};

module.exports = siteConfig;
