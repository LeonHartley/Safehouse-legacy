var userData = {}

var user = JSON.parse(sessionStorage.getItem('ScheduleApp-UserAccountData'))

if (user !== null) {
  userData = user
}

export default {
  get () {
    return userData
  },

  set (data) {
    sessionStorage.setItem('ScheduleApp-UserAccountData', JSON.stringify(data))
    userData = data
  }
}
