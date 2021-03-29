const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
	entry: path.resolve(__dirname, '../index.js'),
	plugins: [
		new HtmlWebpackPlugin({
			template: path.resolve(__dirname, '../index.html'),
		}),
	],
	experiments: {
		asyncWebAssembly: true,
	},
	module: {
		rules: [
			{
				test: /\.html$/i,
				loader: 'html-loader',
			},
		],
	},
};
