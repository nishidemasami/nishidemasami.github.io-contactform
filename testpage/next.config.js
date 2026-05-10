/** @type {import('next').NextConfig} */
const nextConfig = {
  basePath: '/testpage',
  assetPrefix: '/testpage/',
  trailingSlash: true,
  output: 'export',
  reactStrictMode: true,
};

module.exports = nextConfig;
