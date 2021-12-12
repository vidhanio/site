/** @type {import('next').NextConfig} */

module.exports = {
  reactStrictMode: true,
  images: {
    domains: ["i0.wp.com"],
  },
  async rewrites() {
    return [
      {
        source: "/resume",
        destination: "https://resume.vidhan.io/resume",
      },
      {
        source: "/resume/:path*",
        destination: "https://resume.vidhan.io/resume/:path*",
      },
    ];
  },
};
