const state = {
  contacts: [
    { id: 1, name: 'Leon', status: 'online', avatar: 'https://avatars1.githubusercontent.com/u/5290512?s=460&v=4' },
    { id: 2, name: 'Jaxter', status: 'offline', avatar: 'https://avatars2.githubusercontent.com/u/3620463?s=460&v=4' }
  ]
}

const mutations = {
  /* We don't have any mutations yet but this'd be add/remove contacts */
}

const getters = {
  allContacts: state => state.contacts
}

export default {
  state,
  mutations,
  getters
}
