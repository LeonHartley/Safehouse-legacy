// import decodeToken from 'jwt-decode'
// import ElectronStore from 'electron-store'

// var Store = new ElectronStore()

var authData = {
  token: null,
  payload: null
}

// let key = ((Math.random() * 100) + 1) + 'authorisation-token'

// function isTokenValid (payload) {
//   // var seconds = new Date().getTime() / 1000

//   // if (payload.exp <= seconds) {
//   //   return false
//   // }

//   // More checks?
//   return true
// }

export default {
  isAuthenticated () {
    if (authData.token !== null) {
      return true
    }

    return false
    // if (token === null || token === undefined) {
    //   return false
    // }

    // var payload = decodeToken(token)

    // if (!isTokenValid(payload)) {
    //   Store.delete(key)
    //   return false
    // }

    // authData.token = token
    // authData.payload = payload
    // return true
  },

  setAuthentication (token) {
    authData.token = token
  },

  clearAuthentication () {
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
