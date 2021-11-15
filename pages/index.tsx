import React from "react";
import Head from "next/head";
import Image from "next/image";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { IconProp } from "@fortawesome/fontawesome-svg-core";
import {
  faTwitter,
  faLinkedin,
  faGithub,
} from "@fortawesome/free-brands-svg-icons";
import { faEnvelope, faGlobe } from "@fortawesome/free-solid-svg-icons";
import "@fortawesome/fontawesome-svg-core/styles.css";
import "tailwindcss/tailwind.css";
import ImageTest from "../public/imagetest.png";

interface SocialProps {
  icon: IconProp;
  url: string;
}

class Social extends React.Component<SocialProps> {
  render() {
    return (
      <a
        className="flex flex-row items-center justify-center gap-2 text-lg text-blue-500 transition-colors hover:text-blue-400 dark:hover:text-blue-600"
        href={this.props.url}
      >
        <FontAwesomeIcon icon={this.props.icon}></FontAwesomeIcon>
        <div>{this.props.children}</div>
      </a>
    );
  }
}

class GitHubProject extends React.Component<{
  title: string;
  url: string;
  imageSrc: StaticImageData;
}> {
  render() {
    return (
      <a
        className="flex flex-col justify-center max-w-sm gap-4 p-4 text-lg text-white transition-colors border-2 border-white rounded-xl"
        href={this.props.url}
      >
        <div className="flex flex-row items-center gap-4 text-left">
          <FontAwesomeIcon icon={faGithub}></FontAwesomeIcon>
          <h3 className="text-xl">{this.props.title}</h3>
        </div>
        <div className="flex flex-row items-center gap-4 text-left">
          <Image
            src={this.props.imageSrc}
            width={128}
            height={128}
            alt="project"
          />
          talking to walltalking to walltalking to walltalking to walltalking to
          walltalking to walltalking to walltalking to wall
        </div>
      </a>
    );
  }
}

class Home extends React.Component {
  render() {
    return (
      <>
        <Head>
          <title>vidhan</title>
          <link rel="icon" href="favicon.ico" />
          <meta name="viewport" content="width=device-width, initial-scale=1" />

          <meta property="og:title" content="vidhan" />
          <meta property="og:type" content="website" />
          <meta property="og:url" content="https://vidhan.io" />
          <meta property="og:image" content="https://vidhan.io/og.png" />
          <meta
            property="og:description"
            content="vidhan's home on the internet."
          />
        </Head>
        <div className="flex flex-row items-center justify-center w-screen h-screen bg-white dark:bg-black">
          <div className="flex flex-col items-center justify-center gap-8 width">
            <h1 className="font-black text-black text-8xl dark:text-white">
              vidhan
            </h1>
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
            <GitHubProject
              title="Checkpoint"
              url="github.com/vidhanio/checkpoint"
              imageSrc={ImageTest}
            ></GitHubProject>
          </div>
        </div>
      </>
    );
  }
}

export default Home;
