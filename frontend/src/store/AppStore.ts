
import { create } from "zustand"
import { persist, createJSONStorage } from 'zustand/middleware'

interface AppStore {
    pageTitle: string,
    setPageTitle: (pageTitle: string) => void
}

const getInitialPageTitle = () => {

    const path = window.location.pathname.split("/")[1]

    const pages = ["users", "analytics", "instances", "dashboard"]

    if (pages.includes(path)) {
        return path.charAt(0).toUpperCase() + path.slice(1)
    }

    return ""
}

export const useAppStore = create(

    persist<AppStore>((set) => ({

        pageTitle: getInitialPageTitle(),
        setPageTitle: (pageTitle: string) => set({ pageTitle }),
    }),

        {
            name: 'app-store',
            storage: createJSONStorage(() => localStorage)
        },
    ),
)
