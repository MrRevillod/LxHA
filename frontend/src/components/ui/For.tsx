
import React, { Children } from "react"

interface ForProps<T> {
    render: (item: T, index: number) => React.ReactNode,
    of: T[]
}

export const For = <T,>({ render, of }: ForProps<T>): React.ReactElement => {
    return (
        <>
            {Children.toArray(of.map((item, index) => render(item, index)))}
        </>
    )
}
