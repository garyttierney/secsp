/**
 * Copyright (c) 2017-present, Facebook, Inc.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

const React = require('react');

const CompLibrary = require('../../core/CompLibrary.js');

const MarkdownBlock = CompLibrary.MarkdownBlock; /* Used to read markdown */
const Container = CompLibrary.Container;
const GridBlock = CompLibrary.GridBlock;

class HomeSplash extends React.Component {
    render() {
        const {siteConfig, language = ''} = this.props;
        const {baseUrl, docsUrl} = siteConfig;

        const SplashContainer = props => (
            <div className="homeContainer">
                <div className="homeSplashFade">
                    <div className="wrapper homeWrapper">{props.children}</div>
                </div>
            </div>
        );

        const ProjectTitle = () => (
            <h2 className="projectTitle">
                {siteConfig.title}
                <small>{siteConfig.tagline}</small>
            </h2>
        );

        const PromoSection = props => (
            <div className="section promoSection">
                <div className="promoRow">
                    <div className="pluginRowBlock">{props.children}</div>
                </div>
            </div>
        );

        const Button = props => (
            <div className="pluginWrapper buttonWrapper">
                <a className="button" href={props.href} target={props.target}>
                    {props.children}
                </a>
            </div>
        );

        return (
            <SplashContainer>
                <div className="inner">
                    <ProjectTitle siteConfig={siteConfig}/>
                    <PromoSection>
                        <Button href="#try">Try It Out</Button>
                    </PromoSection>
                </div>
            </SplashContainer>
        );
    }
}

class Index extends React.Component {
    render() {
        const {config: siteConfig, language = ''} = this.props;
        const {baseUrl, docsUrl} = siteConfig;
        const docsPart = `${docsUrl ? `${docsUrl}/` : ''}`;
        const langPart = `${language ? `${language}/` : ''}`;
        const docUrl = doc => `${baseUrl}${docsPart}${langPart}${doc}`;

        return (
            <div>
                <HomeSplash siteConfig={siteConfig} language={language}/>
                <div className="mainContainer">
                    <Container>
                        <div className="gridBlock">
                            <div className="twoByGridBlock window">
                                <div className="titlebar">
                                    <div className="titlebar-stoplight">
                                        <div className="titlebar-close">
                                        </div>
                                        <div className="titlebar-minimize">
                                        </div>
                                        <div className="titlebar-fullscreen">
                                        </div>
                                    </div>
                                </div>
                                <div className="blockContent content" id="try">
                                </div>
                            </div>
                            <div className="twoByGridBlock">
                                <div className="blockContent">
                                    <h2>
                                        Rich language tooling
                                    </h2>
                                    <div>
                                        <p><strong>secsp</strong> supports many common text editors using the&nbsp;
                                            <a href="https://microsoft.github.io/language-server-protocol/">Language
                                                Server Protocol</a> and via. custom extensions.
                                            The following editors are officially supported by secsp:</p>

                                        <ul>
                                            <li>Visual Studio Code</li>
                                            <li>Vim/Neovim</li>
                                        </ul>

                                        <p> For more information on using these editors, see the information in the <a
                                            href={docUrl("user")}>User Guide</a></p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </Container>
                </div>
            </div>
        );
    }
}

module.exports = Index;
