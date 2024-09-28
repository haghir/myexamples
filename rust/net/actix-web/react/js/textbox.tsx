type TextBoxProps = {
    id: string,
    label: string,
    value: string,
    onChange: (string) => void,
}

class TextBox extends React.Component<TextBoxProps> {
    constructor(props: TextBoxProps) {
        super(props);
        this.handleChange = this.handleChange.bind(this);
    }

    handleChange(e) {
        this.props.onChange(e.target.value);
    }

    render() {
        console.log("Render TextBox " + this.props.id);
        const id = this.props.id;
        const label = this.props.label;
        const value = this.props.value;
        return (<div>
            <label htmlFor={id}>{label}</label>
            <input id={id} value={value} onChange={this.handleChange}/>
        </div>);
    }
}
