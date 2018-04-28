import decodeToken from 'jwt-decode'

var Session = {
  getCookie () {
    return 'lol'
  }
}

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

    var token = Session.getCookie('auth-token')

    if (token === null || token === undefined) {
      return false
    }

    var payload = decodeToken(token)

    if (!isTokenValid(payload)) {
      Session.removeCookie('auth-token')
      return false
    }

    authData.token = token
    authData.payload = payload

    console.log(authData)
    return true
  },

  setAuthentication (token) {
    Session.addCookie('auth-token', token)
  },

  clearAuthentication () {
    Session.removeCookie('auth-token')

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
