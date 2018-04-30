const state = {
  contacts: [],
  activeContact: null
}

const mutations = {
  setContacts (state, payload) {
    state.contacts = payload.contacts
    state.activeContact = state.contacts[0]
  },

  setActiveContact (state, payload) {
    state.activeContact = payload.activeContact
  },

  updateContacts (state, payload) {
    for (var i = 0; i < state.contacts.length; i++) {
      state.contacts[i].status = payload[i].status.toLowerCase()
    }
  }
}

export default {
  state,
  mutations
}
