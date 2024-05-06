
import { create } from 'zustand'
import { User } from '../lib/types'

interface UserStore {
    user: User | null
    isAdmin: boolean
}

export const useUserStore = create<UserStore>(() => ({

    user: null,
    isAdmin: false,

}))
