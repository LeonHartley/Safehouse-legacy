import decodeToken from 'jwt-decode'
import Store from 'electron-store'

var authData = {
  token: null,
  payload: null
}

function isTokenValid (payload) {
  var seconds = new Date().getTime() / 1000

  if (payload.exp <= seconds) {
    return false
  }

  // More checks?

  return true
}

export default {
  isAuthenticated () {
    if (authData.token !== null) {
      return true
    }

    var token = Store.get('auth-token')

    if (token === null || token === undefined) {
      return false
    }

    var payload = decodeToken(token)

    if (!isTokenValid(payload)) {
      Store.delete('auth-token')
      return false
    }

    authData.token = token
    authData.payload = payload
    return true
  },

  setAuthentication (token) {
    Store.set('auth-token', token)
  },

  clearAuthentication () {
    Store.delete('auth-token')

    authData.token = null
    authData.payload = null
  },

  getAuthPayload () {
    return authData.payload
  },

  getAuthToken () {
    return authData.token
  }
}
