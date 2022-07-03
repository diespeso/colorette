'use strict'

import Clock from './clock.js'

class LikeButton extends React.Component {
    constructor(props) {
        super(props)
        this.state = {liked: false}

        this.handleClick = this.handleClick.bind(this)
    }

    handleClick() {
        this.setState({liked: !this.state.liked})
    }

    render() {
        console.log(`stado render: ${this.state.liked}`)
        if(this.state.liked) {
            console.log('bad')
            return (<p>Likeado</p>)
        }

        return (
            <button onClick={() => {this.handleClick(); this.props.onClick()}}>press me</button>
        )
    }
}

export default LikeButton
/*
const container = document.getElementById('like_button_container')
const root = ReactDOM.createRoot(container)
root.render(<Clock />)*/