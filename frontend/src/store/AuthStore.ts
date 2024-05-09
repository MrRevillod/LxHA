
import { api } from "../lib/axios"
import { create } from "zustand"
import { useUserStore } from "./UserStore"
import { useHttpStore } from "./HttpStore"
import { RequestResetPasswordData, LoginData } from "../lib/types"

// TODO! Replace any types

interface AuthStore {
    isAuthenticated: boolean,
    isAdmin: boolean,
    useLogin: (data: LoginData) => Promise<void>,
    useLogout: () => Promise<void>
    useValidateSession: () => Promise<void>,
    useValidatePermisions: () => Promise<void>,
    useValidateAccount: (id: string | undefined, token: string | undefined) => Promise<void>,
    useRequestResetPassword: (body: RequestResetPasswordData) => Promise<void>,
    useValidateResetPasswordPage: (id: string | undefined, token: string | undefined) => Promise<void>,
    useResetPassword: (id: string | undefined, token: string | undefined, body: any) => Promise<void>
}

export const useAuthStore = create<AuthStore>((set) => ({

    isAuthenticated: false,
    isAdmin: false,

    useLogin: async (data: LoginData) => {

        try {

            const res = await api.post("/auth/login", data)
            set({ isAuthenticated: res.status === 200 })

            useUserStore.setState({ user: res.data?.user })
            useHttpStore.setState({ status: res.status, message: res.data?.message })

        } catch (error: any) { 

            set({ isAuthenticated: false })
            useUserStore.setState({ user: null })

            useHttpStore.setState({ 
                status: error.response.status, 
                message: error.response.data?.message}
            )
        }
    },

    useLogout: async () => {

        try {

            const res = await api.post("/auth/logout")
            set({ isAuthenticated: !(res.status === 200) })

            useHttpStore.setState({ status: res.status, message: res.data?.message })

        } catch (error: any) { 

            set({ isAuthenticated: false })
            useUserStore.setState({ user: null })

            useHttpStore.setState({ 
                status: error.response.status, 
                message: error.response.data?.message}
            )
        }
    },

    useValidateSession: async () => {

        try {

            const res = await api.post("/auth/validate-session")
            set({ isAuthenticated: res.status === 200 })
            
            useHttpStore.setState({ status: res.status, message: res.data?.message })

        } catch (error: any) { 

            set({ isAuthenticated: false })
            useUserStore.setState({ user: null })

            useHttpStore.setState({ 
                status: error.response.status, 
                message: error.response.data?.message}
            )
        }
    },

    useValidatePermisions: async () => {
        
        try {

            const res = await api.post("/auth/validate-role")

            if (res.status === 200) set({ isAdmin: true })

            useHttpStore.setState({ status: res.status, message: res.data?.message })

        } catch (error: any) {

            useUserStore.setState({ isAdmin: false })
            
            useHttpStore.setState({ 
                status: error.response.status, 
                message: error.response.data?.message}
            )
        }
    },

    useValidateAccount: async (id, token) => {

        try {

            const res = await api.post(`/auth/validate/${id}/${token}`)
            useHttpStore.setState({ status: res.status, message: res.data?.message })

        } catch (error: any) { 
            
            useHttpStore.setState({ 
                status: error.response.status, 
                message: error.response.data?.message}
            )
        }
    },

    useRequestResetPassword: async (body: RequestResetPasswordData) => {
        
        try {

            const res =  await api.post("/auth/reset-password", body)
            useHttpStore.setState({ status: res.status, message: res.data?.message })

        } catch (error: any) { 
            
            useHttpStore.setState({ 
                status: error.response.status, 
                message: error.response.data?.message}
            )
        }
    },

    useValidateResetPasswordPage: async (id, token) => {
        
        try {

            const res = await api.post(`/auth/reset-password/${id}/${token}`)
            useHttpStore.setState({ status: res.status, message: res.data?.message })

        } catch (error: any) { 
            
            useHttpStore.setState({ 
                status: error.response.status, 
                message: error.response.data?.message}
            )
        }
    },

    useResetPassword: async (id, token, body) => {
        
        try {

            const res = await api.patch(`/auth/reset-password/${id}/${token}`, body)
            useHttpStore.setState({ status: res.status, message: res.data?.message })

        } catch (error: any) { 
            
            useHttpStore.setState({ 
                status: error.response.status, 
                message: error.response.data?.message}
            )
        }
    },
}))
