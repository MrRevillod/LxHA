
import { api } from "../lib/axios"
import { create } from "zustand"
import { ROLE, RegisterData, User } from "../lib/types"

interface UserStore {

    user: User | null,
    setUser: (user: User | null) => void,
    createUser: (userData:RegisterData) => Promise<void>,
    updateUser: () => Promise<void>,
    deleteUser: (id: string) => Promise<void>,
}

export const  useUserStore = create<UserStore>((set) => ({

    user: null,

    setUser: (user: User | null) => set({ user }),

    createUser: async (userData : RegisterData) => {

        // const userData: RegisterData = {
        //     email: "mail_test@mail.com",
        //     username: "test_user",
        //     role: ROLE.ADMINISTRATOR,
        //     password: "aaa",
        //     confirmPassword: "aaa"
        // }

        console.log("userData: ", userData)
        try {

            await api.post("/dashboard/user/register-account", userData)

        } catch (error: any) {
            console.error(error)
        }
    },

    updateUser: async () => {

        const id = "664ba271d0c2f9a2d17bbea0" // replace with your user id
        const updateFields = {
            password: "abc",
            confirmPassword: "abc"
        }

        try {

            await api.patch(`/dashboard/user/update-account/${id}`, updateFields)

        } catch (error: any) {

            console.error(error)
        }
    },

    deleteUser: async (id: string) => {

        try {

            await api.delete(`/dashboard/user/delete-account/${id}`)

        } catch (error: any) {

            console.error(error)
        }
    }

}))
