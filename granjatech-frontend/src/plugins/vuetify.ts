import 'vuetify/styles'
import '@mdi/font/css/materialdesignicons.css'
import '@fontsource/inter/400.css'
import '@fontsource/inter/500.css'
import '@fontsource/inter/600.css'
import '@fontsource/inter/700.css'

import { createVuetify } from 'vuetify'

const lightTheme = {
  dark: false,
  colors: {
    primary: '#2E7D32',
    'primary-lighten-1': '#66BB6A',
    'primary-darken-1': '#1B5E20',
    secondary: '#FF6F00',
    'secondary-lighten-1': '#FFB74D',
    'secondary-darken-1': '#E65100',
    background: '#F8F9FA',
    surface: '#FFFFFF',
    success: '#4CAF50',
    error: '#F44336',
    warning: '#FF9800',
    info: '#2196F3',
  },
}

const darkTheme = {
  dark: true,
  colors: {
    primary: '#2E7D32',
    'primary-lighten-1': '#66BB6A',
    'primary-darken-1': '#1B5E20',
    secondary: '#FF6F00',
    'secondary-lighten-1': '#FFB74D',
    'secondary-darken-1': '#E65100',
    background: '#121212',
    surface: '#1E1E1E',
    success: '#4CAF50',
    error: '#F44336',
    warning: '#FF9800',
    info: '#2196F3',
  },
}

export default createVuetify({
  theme: {
    defaultTheme: 'light',
    themes: {
      light: lightTheme,
      dark: darkTheme,
    },
  },
  defaults: {
    VBtn: {
      rounded: 'lg',
    },
    VCard: {
      rounded: 'lg',
    },
    VTextField: {
      variant: 'outlined',
      rounded: 'lg',
    },
    VAppBar: {
      flat: true,
    },
  },
})
