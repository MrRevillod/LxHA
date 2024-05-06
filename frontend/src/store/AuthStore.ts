
import { create } from 'zustand'

import { api } from '../lib/axios'
import { loginData } from '../lib/types'
import { useUserStore } from './UserStore'

interface AuthStore {
    isAuthenticated: boolean,
    isAdmin: boolean,
    useLogin: (data: loginData) => Promise<void>,
    useLogout: () => Promise<void>,
    useValidateSession: () => Promise<void>,
    useValidatePermisions: () => Promise<void>,
    useValidateAccount: (id: string, token: string) => Promise<void>
}

export const useAuthStore = create<AuthStore>((set) => ({

    isLoading: false,
    isAuthenticated: false,
    isAdmin: false,

    useLogin: async (data: loginData) => {

        try {

            const res = await api.post("/auth/login", data)
            set({ isAuthenticated: res.status === 200 })

            useUserStore.setState({ user: res.data?.user })

        } catch {
            
            useUserStore.setState({ user: null })
            set({ isAuthenticated: false })
        }
    },

    useLogout: async () => {

        try {
            const res = await api.post("/auth/logout")
            set({ isAuthenticated: !(res.status === 200) })

        } catch { 
            useUserStore.setState({ user: null })
            set({ isAuthenticated: false })
        }
    },

    useValidateSession: async () => {

        try {
            const res = await api.post("/auth/validate-session")
            set({ isAuthenticated: res.status === 200 })

        } catch { 
            useUserStore.setState({ user: null })
            set({ isAuthenticated: false })
        }
    },

    useValidatePermisions: async () => {
        
        try {

            const res = await api.post("/auth/validate-role")

            if (res.status === 200) {
                set({ isAdmin: true })
            }

        } catch {
            useUserStore.setState({ isAdmin: false })
        }
    },

    useValidateAccount: async (id, token) => {

        try {

            const res = await api.post(`/auth/validate/${id}/${token}`)

            if (res.status === 200) {
                console.log(res.data?.message)
            }


        } catch { return }
    }

}))
