/** @type {import('tailwindcss').Config} */
module.exports = {
	content: ["index.html", "./src/*.rs*", "./src/**/*.{rs,html,js,jsx,ts,tsx}"],
	darkMode: "class",
	theme: {
		extend: {
			colors: {
				background: "var(--color-bg)",
				text: "var(--color-text)",
			},
		},
	},
	plugins: [],
};
