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
  }
}

export default {
  state,
  mutations
}
