import React from 'react';

class Page extends React.Component {
    constructor(props) {
        super(props)
        this.state = {
            blocks:[]
        }
    }

    componentDidMount() {
        fetch('http://localhost:8000/page/'+this.props.page, {
            method: "GET",
            headers: {
                'Accept': 'application/json',
                "Content-Type": "application/json"
            },
        })
            .then((response) => response.json())
            .then((data) => {
                this.setState({ blocks: data.blocks });
            })
            .catch((err) => {
                console.log(err.message);
            })
    };

    renderJSON(block, key) {
        switch (block.type) {
            case 'paragraph':
                return <p dangerouslySetInnerHTML={{ __html: block.data.text }} />;
            case 'header':
                const HeadingTag = `h${block.data.level}`;
                return <HeadingTag dangerouslySetInnerHTML={{ __html: block.data.text }} />;
            case 'list':
                let ListTag;
                let ItemTag;
                if (block.style === 'ordered') {
                    ListTag = `ol`;
                    ItemTag = 'li';
                } else {
                    ListTag = `ul`;
                    ItemTag = 'li';
                }

                return (
                    <ListTag>
                        {block.data.items.map((item, index) =>
                            <ItemTag dangerouslySetInnerHTML={{ __html: item }} />
                        )}
                    </ListTag>
                );
            default:
                return;
        }
    }

    render() {
        return (
            <main>
                { this.state.blocks.map(((block, index) => this.renderJSON(block, index))) }
            </main>
        );
    };
};
  
export default Page;