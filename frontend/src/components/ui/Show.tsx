
import React from "react"

interface Props {
    when: boolean,
    children: React.ReactNode
}

export const Show = ({ when, children }: Props) => {
    return when ? children : null
}
