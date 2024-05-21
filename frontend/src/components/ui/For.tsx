
import { Children } from "react"

interface Props {
    render: (item: React.ReactNode, index: number) => React.ReactNode,
    of: any[],
}

export const For = ({ render, of }: Props) => {
    Children.toArray(of.map((item, index) => render(item, index)))
}