const { defineConfig } = require('@vue/cli-service')
module.exports = defineConfig({
  publicPath: './',
  transpileDependencies: [
    'vuetify'
  ],
  devServer: {
    port: 8081,
    proxy: {
      '/': {
        target: process.env.VUE_APP_LOCAL_PROXY_DOMAIN,
        ws: false,
      }
    }
  }
})
