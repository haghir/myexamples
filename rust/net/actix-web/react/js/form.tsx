type FormProps = {
    name: string,
    age: string,
};

type FormState = {
    name: string,
    age: string,
};

class Form extends React.Component<FormProps, FormState> {
    constructor(props) {
        super(props);
        this.handleSubmit = this.handleSubmit.bind(this);
        this.handleNameChange = this.handleNameChange.bind(this);
        this.handleAgeChange = this.handleAgeChange.bind(this);
        this.state = {
            name: props.name,
            age: props.age
        };
    }

    handleSubmit(e) {
        e.preventDefault();
        alert(this.state.name + ", " + this.state.age);
    }

    handleNameChange(name) {
        this.setState({ name: name });
    }

    handleAgeChange(age) {
        this.setState({ age: age });
    }

    render() {
        console.log("Render Form");
        return (
        <form onSubmit={this.handleSubmit}>
            <TextBox id="name" label="Name" value={this.state.name} onChange={this.handleNameChange}/>
            <TextBox id="age" label="Age" value={this.state.age} onChange={this.handleAgeChange}/>
            <input type="submit"/>
        </form>);
    }
}
