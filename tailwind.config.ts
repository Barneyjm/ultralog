import type { Config } from 'tailwindcss';
import defaultTheme from 'tailwindcss/defaultTheme';

export default <Partial<Config>>{
  theme: {
    extend: {
      fontFamily: {
        sans: ['DM Sans', ...defaultTheme.fontFamily.sans],
      },
      colors: {
        haltech: '#F5C147',
        linkecu: '#3A1B4E',
        maxx: '#DA4D32',
      },
    },
  },
};
