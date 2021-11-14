import React from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { IconProp } from "@fortawesome/fontawesome-svg-core";
import {
  faTwitter,
  faLinkedin,
  faGithub,
} from "@fortawesome/free-brands-svg-icons";
import { faEnvelope, faGlobe } from "@fortawesome/free-solid-svg-icons";

interface SocialProps {
  icon: IconProp;
  url: string;
}

class Social extends React.Component<SocialProps> {
  render() {
    return (
      <a
        className="flex flex-row items-center justify-center gap-2 text-lg text-blue-500 transition-all hover:text-blue-400"
        href={this.props.url}
      >
        <FontAwesomeIcon icon={this.props.icon}></FontAwesomeIcon>
        <div>{this.props.children}</div>
      </a>
    );
  }
}

class Home extends React.Component {
  render() {
    return (
      <div className="flex flex-row w-screen h-screen justify-center items-center">
        <div className="flex flex-col justify-center items-center width gap-8">
          <h1 className="text-8xl font-black">vidhan</h1>
          <div className="flex flex-row gap-4 font-bold">
            <Social icon={faEnvelope} url="mailto:me@vidhan.io">
              me@vidhan.io
            </Social>
            <Social icon={faGithub} url="https://github.com/vidhanio">
              vidhanio
            </Social>
            <Social icon={faLinkedin} url="https://linkedin.com/in/vidhanio">
              vidhanio
            </Social>
            <Social icon={faTwitter} url="https://twitter.com/vidhanio">
              vidhanio
            </Social>
            <Social icon={faGlobe} url="https://vidhan.io">
              vidhan.io
            </Social>
          </div>
        </div>
      </div>
    );
  }
}

export default Home;
