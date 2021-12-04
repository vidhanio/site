import React from "react";
import Image from "next/image";
import Link from "next/link";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { IconProp } from "@fortawesome/fontawesome-svg-core";
import "@fortawesome/fontawesome-svg-core/styles.css";
import {
  faTwitter,
  faLinkedin,
  faGithub,
} from "@fortawesome/free-brands-svg-icons";
import {
  faEnvelope,
  faGlobe,
  faArrowDown,
} from "@fortawesome/free-solid-svg-icons";
import CheckpointImage from "../public/checkpoint.png";
import VirtualBoxImage from "../public/virtualbox.png";
import TripleTacToeImage from "../public/triple-tac-toe.png";

interface SocialProps {
  icon: IconProp;
  url: string;
  name: string;
}

interface GitHubProjectProps {
  url: string;
  imageSrc: string | StaticImageData;
  body: string;
}

function Social({ icon, url, name }: SocialProps) {
  return (
    <a
      className="flex flex-row gap-2 justify-center items-center text-lg text-blue-500 transition-colors hover:text-blue-400 dark:hover:text-blue-600"
      href={url}
    >
      <FontAwesomeIcon icon={icon}></FontAwesomeIcon>
      <div>{name}</div>
    </a>
  );
}

function GitHubProject({ url, imageSrc, body }: GitHubProjectProps) {
  return (
    <a href={url}>
      <div className="flex flex-col gap-4 justify-center items-center p-8 w-96 text-lg text-black bg-white rounded-xl md:w-auto md:max-h-64">
        <div className="flex flex-row gap-2 items-center">
          <FontAwesomeIcon icon={faGithub}></FontAwesomeIcon>
          <div className="text-sm">/</div>
          <h3 className="text-xl font-bold">{url.split("/").pop()}</h3>
        </div>
        <div className="flex flex-col gap-4 items-center text-center md:text-left md:flex-row">
          <div className="w-32 h-32">
            <Image
              src={imageSrc}
              alt="project"
              layout="responsive"
              className="rounded-lg shadow-xl"
            />
          </div>
          <div className="w-64">{body}</div>
        </div>
      </div>
    </a>
  );
}

interface SectionProps {
  title: string;
  subtitle?: React.ReactNode;
  children: React.ReactNode;
}

function Section({ title, subtitle, children }: SectionProps) {
  return (
    <div className="flex flex-col gap-16 justify-center items-center p-16 min-h-screen">
      <h3 className="text-8xl font-black text-black dark:text-white">
        {title}
      </h3>
      {children}
    </div>
  );
}

class Home extends React.Component {
  render() {
    return (
      <div className="flex flex-col justify-center items-center w-screen min-h-screen">
        <Section title="vidhan">
          <div className="flex flex-col gap-2 font-bold md:flex-row md:gap-4">
            <Social
              icon={faEnvelope}
              url="mailto:me@vidhan.io"
              name="me@vidhan.io"
            ></Social>
            <Social
              icon={faGithub}
              url="https://github.com/vidhanio"
              name="vidhanio"
            ></Social>
            <Social
              icon={faLinkedin}
              url="https://linkedin.com/in/vidhanio"
              name="vidhanio"
            ></Social>
            <Social
              icon={faTwitter}
              url="https://twitter.com/vidhanio"
              name="vidhanio"
            ></Social>
          </div>
          <a
            href="#projects"
            className="text-4xl text-white transition-colors hover:text-blue-500"
          >
            <FontAwesomeIcon icon={faArrowDown}></FontAwesomeIcon>{" "}
          </a>
        </Section>
        <div
          id="projects"
          className="flex flex-col gap-16 justify-center items-center p-16 min-h-screen"
        >
          <h1 className="text-8xl font-black text-black dark:text-white">
            projects
          </h1>{" "}
          <div className="flex flex-col flex-wrap gap-4 justify-center items-center md:flex-row">
            <GitHubProject
              url="https://github.com/vidhanio/checkpoint"
              imageSrc={CheckpointImage}
              body="A discord verification bot for servers in the Peel District School Board."
            ></GitHubProject>
            <GitHubProject
              url="https://github.com/vidhanio/virtualbox-rich-presence"
              imageSrc={VirtualBoxImage}
              body="A discord rich presence for VirtualBox."
            ></GitHubProject>
            <GitHubProject
              url="https://github.com/vidhanio/triple-tac-toe"
              imageSrc={TripleTacToeImage}
              body="A simple game of triple tac toe."
            ></GitHubProject>
          </div>
        </div>
      </div>
    );
  }
}

export default Home;
