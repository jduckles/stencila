const globby = require('globby')
const path = require('path')
const pkgJson = require('./package.json')

const MiniCssExtractPlugin = require('mini-css-extract-plugin')
const FileManagerPlugin = require('filemanager-webpack-plugin')
const { CleanWebpackPlugin } = require('clean-webpack-plugin')
const HtmlWebpackPlugin = require('html-webpack-plugin')
const ScriptExtHtmlWebpackPlugin = require('script-ext-html-webpack-plugin')
const { DefinePlugin, HotModuleReplacementPlugin } = require('webpack')

// TODO: Explore converting Webpack configuration to TypeScipt, to allow importing of theme names
const themes = [
  'bootstrap',
  'elife',
  'nature',
  'plos',
  'rpng',
  'skeleton',
  'stencila',
  'wilmore'
]

// Generate configurations for ScriptExtHtmlWebpackPlugin to add theme ID and class attributes
// to Stylesheet imports for documentation site. This is needed for the Theme switcher to function.
const themePaths = themes.map(theme => `themes/${theme}/styles`)
const themeIds = themes.map(theme => ({
  test: `themes/${theme}/styles`,
  attribute: 'id',
  value: theme
}))

const contentSource = 'src'

// Convert absolute filepaths to project relative ones to use as
// output destinations.
const makeRelativePath = filepath =>
  path.relative(path.join(__dirname, contentSource), filepath)

// Strip `/src` from output destination pathnames.
// Otherwise Webpack outputs files at `/dist/src/*`
const fileLoaderOutputPath = (url, resourcePath, context) => {
  const relativePath = path
    .relative(context, resourcePath)
    .replace(`${contentSource}/`, '')

  return `${relativePath}`
}

module.exports = (env = {}, { mode }) => {
  const isDocs = env.docs === 'true'
  const isDevelopment = mode === 'development'
  const contentBase = isDocs ? 'docs' : 'dist'

  const entries = [
    './src/**/*.{css,ts,html,ttf,woff,woff2}',
    // template.html is used as a basis for HtmlWebpackPlugin, and should not be used as an entry point
    '!./src/template.html',
    // Don’t compile test files for package distribution
    '!**/*.{d,test}.ts',
    // These files make use of Node APIs, and do not need to be packaged for Browser targets
    '!**/scripts/*.ts',
    '!**/lib/**/*.ts',
    '!**/extensions/math/update.ts',
    '!**/extensions/extensions.ts',
    // Don’t build HTML demo files for package distribution
    ...(isDocs || isDevelopment
      ? []
      : ['!**/*.html', '!**/demo/*', '!**/examples/*'])
  ]

  const entry = globby.sync(entries).reduce(
    (files, file) => ({
      ...files,
      [makeRelativePath(file)
        .replace(/.ts$/, '')
        .replace(/.css$/, '')]: file
    }),
    {}
  )

  // Only generate HTML files for documentation builds, and local development
  const docsPlugins =
    isDocs || isDevelopment
      ? [
          new HtmlWebpackPlugin({
            template: './src/template.html',
            chunks: ['demo/index', 'demo/styles', ...themePaths]
          }),
          new ScriptExtHtmlWebpackPlugin({
            custom: [
              {
                test: 'themes/',
                attribute: 'disabled'
              },
              {
                test: 'themes/',
                attribute: 'class',
                value: 'theme'
              },
              ...themeIds
            ]
          })
        ]
      : []

  return {
    entry,
    resolve: {
      extensions: ['.ts', '.tsx', '.js', '.css', '.html']
    },
    mode: mode || 'development',
    output: {
      path: path.resolve(__dirname, contentBase),
      filename: '[name].js'
    },
    devServer: {
      contentBase: `./${contentBase}`
    },
    plugins: [
      new CleanWebpackPlugin(),
      new DefinePlugin({
        'process.env.NODE_ENV': JSON.stringify(process.env.NODE_ENV),
        'process.env.VERSION': JSON.stringify(
          process.env.VERSION || pkgJson.version
        )
      }),
      new MiniCssExtractPlugin(),
      ...docsPlugins,
      // After a successful build, delete empty artifacts generated by Webpack for
      // non TypeScript/JavaScript files (i.e. for font and CSS files).
      new FileManagerPlugin({
        onEnd: {
          delete: [
            `${contentBase}/**/styles.js`,
            `${contentBase}/fonts/**/*.js`,
            `${contentBase}/generate`,
            ...(isDocs ? [] : [`${contentBase}/demo/`, `${contentBase}/share/`])
          ]
        }
      })
    ],
    module: {
      rules: [
        {
          test: /\.tsx?$/,
          use: {
            loader: 'ts-loader',
            options: {
              configFile: 'tsconfig.browser.json',
              experimentalWatchApi: true
            }
          }
        },
        {
          test: /\.html$/i,
          // Don't transform HtmlWebpackPlugin generated file
          exclude: /template\.html$/i,
          use: [
            {
              loader: 'file-loader',
              options: {
                name: '[name].[ext]',
                outputPath: fileLoaderOutputPath
              }
            },
            'extract-loader',
            'html-loader'
          ]
        },
        {
          test: /\.(css)$/,
          use: [
            {
              loader: MiniCssExtractPlugin.loader,
              options: { hmr: isDevelopment }
            },
            {
              loader: 'css-loader',
              options: { importLoaders: 1, url: false, import: true }
            },
            'postcss-loader'
          ]
        },
        {
          test: /\.(eot|woff|woff2|svg|ttf|jpe?g|png|gif)$|html\.media\/.*$/,
          use: [
            {
              loader: 'file-loader',
              options: {
                name: '[folder]/[name].[ext]',
                outputPath: fileLoaderOutputPath
              }
            }
          ]
        }
      ]
    }
  }
}
