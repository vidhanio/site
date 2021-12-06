/** @type {import('next').NextConfig} */
module.exports = {
  reactStrictMode: true,
  basePath: "/triple-tac-toe",
  async rewrites() {
    return [
      {
        source: "/",
        destination: "/triple-tac-toe",
      },
      {
        source: "/:path*",
        destination: "/triple-tac-toe/:path*",
      },
    ];
  },
};
