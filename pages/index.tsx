import React from "react";
import _ from "lodash";
import Image from "next/image";
import x from "../public/images/x.png";
import o from "../public/images/o.png";
import t from "../public/images/t.png";

type Winner = true | false | null | undefined;
type SmallBoard = Winner[][];
type MediumBoard = SmallBoard[][];
type LargeBoard = MediumBoard[][];

interface TTTProps {
  coords: [number, number];
  turn: true | false;
  winner: Winner;
  active: boolean;
}

interface BoxProps extends TTTProps {
  onFinishTurn: (boxProps: BoxProps) => void;
  onPreviewTurn: (boxProps: BoxProps) => void;
  onUnpreviewTurn: (boxProps: BoxProps) => void;
  previewed: boolean;
}

interface SmallProps extends TTTProps {
  board: SmallBoard;
  onFinishTurn: (smallProps: SmallProps, boxProps: BoxProps) => void;
  onPreviewTurn: (smallProps: SmallProps, boxProps: BoxProps) => void;
  onUnpreviewTurn: (smallProps: SmallProps, boxProps: BoxProps) => void;
  previewed: boolean;
}

interface MediumProps extends TTTProps {
  board: MediumBoard;
  onFinishTurn: (
    mediumProps: MediumProps,
    smallProps: SmallProps,
    boxProps: BoxProps
  ) => void;
  activeCoords: [number, number];
  onPreviewTurn: (
    mediumProps: MediumProps,
    smallProps: SmallProps,
    boxProps: BoxProps
  ) => void;
  onUnpreviewTurn: (
    mediumProps: MediumProps,
    smallProps: SmallProps,
    boxProps: BoxProps
  ) => void;
  previewCoords?: [number, number];
}

interface LargeState {
  board: LargeBoard;
  winner: Winner;
  turn: true | false;
  activeCoords: [number, number];
  activeSmallCoords: [number, number];
  previewCoords?: [number, number];
  previewSmallCoords?: [number, number];
}

function genBoard(
  size: "large" | "medium" | "small",
  fill: Winner
): LargeBoard | MediumBoard | SmallBoard {
  switch (size) {
    case "large":
      return _.times(3, () =>
        _.times(3, () =>
          _.times(3, () =>
            _.times(3, () => _.times(3, () => _.times(3, () => fill)))
          )
        )
      );
    case "medium":
      return _.times(3, () =>
        _.times(3, () => _.times(3, () => _.times(3, () => fill)))
      );
    case "small":
      return _.times(3, () => _.times(3, () => fill));
  }
}

class Box extends React.Component<BoxProps> {
  constructor(props: BoxProps) {
    super(props);
    this.finishTurn = this.finishTurn.bind(this);
    this.previewTurn = this.previewTurn.bind(this);
    this.unpreviewTurn = this.unpreviewTurn.bind(this);
  }

  finishTurn() {
    this.props.onFinishTurn(this.props);
  }

  previewTurn(e: React.MouseEvent<HTMLButtonElement>) {
    this.props.onPreviewTurn(this.props);
  }

  unpreviewTurn(e: React.MouseEvent<HTMLButtonElement>) {
    this.props.onUnpreviewTurn(this.props);
  }

  render() {
    return (
      <button
        className={`font-mono transition-colors w-4 h-4 text-xs rounded-sm ${
          this.props.winner !== undefined && this.props.active
            ? "bg-blue-300"
            : this.props.active
            ? "bg-blue-400"
            : this.props.winner !== undefined
            ? "bg-gray-200"
            : "bg-gray-300"
        } ${this.props.previewed ? "bg-green-300" : ""}`}
        onClick={
          this.props.winner === undefined || this.props.active
            ? this.finishTurn
            : () => undefined
        }
        onMouseEnter={
          this.props.winner === undefined && this.props.active
            ? this.previewTurn
            : () => undefined
        }
        onMouseLeave={
          this.props.winner === undefined && this.props.active
            ? this.unpreviewTurn
            : () => undefined
        }
      >
        {this.props.winner === undefined ? (
          ""
        ) : this.props.winner === null ? (
          <Image src={t} alt="t" />
        ) : this.props.winner ? (
          <Image src={x} alt="x" />
        ) : (
          <Image src={o} alt="o" />
        )}
      </button>
    );
  }
}

class SmallTTT extends React.Component<SmallProps> {
  constructor(props: SmallProps) {
    super(props);

    this.finishTurn = this.finishTurn.bind(this);
    this.previewTurn = this.previewTurn.bind(this);
    this.unpreviewTurn = this.unpreviewTurn.bind(this);
    this.renderBox = this.renderBox.bind(this);
  }

  finishTurn(boxProps: BoxProps) {
    const newBoard = this.props.board;
    newBoard[boxProps.coords[1]][boxProps.coords[0]] = this.props.turn;
    this.props.onFinishTurn(this.props, boxProps);
  }

  previewTurn(boxProps: BoxProps) {
    this.props.onPreviewTurn(this.props, boxProps);
  }

  unpreviewTurn(boxProps: BoxProps) {
    this.props.onUnpreviewTurn(this.props, boxProps);
  }

  renderBox(coords: [number, number]) {
    return (
      <Box
        previewed={this.props.previewed}
        onPreviewTurn={this.previewTurn}
        onUnpreviewTurn={this.unpreviewTurn}
        onFinishTurn={this.finishTurn}
        coords={coords}
        turn={this.props.turn}
        winner={this.props.board[coords[1]][coords[0]]}
        active={this.props.active}
      />
    );
  }

  render() {
    return (
      <div className="flex flex-col gap-1">
        <div className="flex flex-row gap-1">
          {this.renderBox([0, 0])}
          {this.renderBox([1, 0])}
          {this.renderBox([2, 0])}
        </div>
        <div className="flex flex-row gap-1">
          {this.renderBox([0, 1])}
          {this.renderBox([1, 1])}
          {this.renderBox([2, 1])}
        </div>
        <div className="flex flex-row gap-1">
          {this.renderBox([0, 2])}
          {this.renderBox([1, 2])}
          {this.renderBox([2, 2])}
        </div>
      </div>
    );
  }
}

class MediumTTT extends React.Component<MediumProps, {}> {
  constructor(props: MediumProps) {
    super(props);

    this.finishTurn = this.finishTurn.bind(this);
    this.previewTurn = this.previewTurn.bind(this);
    this.unpreviewTurn = this.unpreviewTurn.bind(this);
    this.renderSmall = this.renderSmall.bind(this);
  }

  finishTurn(smallProps: SmallProps, boxProps: BoxProps) {
    this.props.onFinishTurn(this.props, smallProps, boxProps);
  }

  previewTurn(smallProps: SmallProps, boxProps: BoxProps) {
    this.props.onPreviewTurn(this.props, smallProps, boxProps);
  }

  unpreviewTurn(smallProps: SmallProps, boxProps: BoxProps) {
    this.props.onUnpreviewTurn(this.props, smallProps, boxProps);
  }

  renderSmall(coords: [number, number]) {
    return (
      <SmallTTT
        previewed={
          this.props.previewCoords !== undefined
            ? this.props.previewCoords[0] === coords[0] &&
              this.props.previewCoords[1] === coords[1]
            : false
        }
        onPreviewTurn={this.previewTurn}
        onUnpreviewTurn={this.unpreviewTurn}
        onFinishTurn={this.finishTurn}
        coords={coords}
        turn={this.props.turn}
        winner={undefined}
        active={
          this.props.active &&
          this.props.activeCoords[0] === coords[0] &&
          this.props.activeCoords[1] === coords[1]
        }
        board={this.props.board[coords[1]][coords[0]]}
      />
    );
  }

  render() {
    return (
      <div className="flex flex-col gap-2">
        <div className="flex flex-row gap-2">
          {this.renderSmall([0, 0])}
          {this.renderSmall([1, 0])}
          {this.renderSmall([2, 0])}
        </div>
        <div className="flex flex-row gap-2">
          {this.renderSmall([0, 1])}
          {this.renderSmall([1, 1])}
          {this.renderSmall([2, 1])}
        </div>
        <div className="flex flex-row gap-2">
          {this.renderSmall([0, 2])}
          {this.renderSmall([1, 2])}
          {this.renderSmall([2, 2])}
        </div>
      </div>
    );
  }
}

class LargeTTT extends React.Component<{}, LargeState> {
  constructor(props: {}) {
    super(props);

    this.state = {
      previewCoords: [0, 0],
      activeCoords: [1, 1],
      activeSmallCoords: [1, 1],
      board: genBoard("large", undefined) as LargeBoard,
      winner: undefined,
      turn: true,
    };

    this.finishTurn = this.finishTurn.bind(this);
    this.previewTurn = this.previewTurn.bind(this);
    this.unpreviewTurn = this.unpreviewTurn.bind(this);
    this.setLargeWinner = this.setLargeWinner.bind(this);
    this.setMediumWinner = this.setMediumWinner.bind(this);
    this.setSmallWinner = this.setSmallWinner.bind(this);
    this.checkSmallWinner = this.checkSmallWinner.bind(this);
  }

  finishTurn(
    mediumProps: MediumProps,
    smallProps: SmallProps,
    boxProps: BoxProps
  ) {
    const board = this.state.board;
    board[mediumProps.coords[1]][mediumProps.coords[0]][smallProps.coords[1]][
      smallProps.coords[0]
    ][boxProps.coords[1]][boxProps.coords[0]] = this.state.turn;
    this.setState({
      board: board,
      activeCoords: smallProps.coords,
      activeSmallCoords: boxProps.coords,
    });

    this.setSmallWinner(mediumProps, smallProps);
    this.setMediumWinner(mediumProps);
    this.setLargeWinner();

    this.setState({ turn: !this.state.turn });
  }

  previewTurn(
    mediumProps: MediumProps,
    smallProps: SmallProps,
    boxProps: BoxProps
  ) {
    this.setState({
      previewCoords: smallProps.coords,
      previewSmallCoords: boxProps.coords,
    });
  }

  unpreviewTurn(
    mediumProps: MediumProps,
    smallProps: SmallProps,
    boxProps: BoxProps
  ) {
    this.setState({
      previewCoords: undefined,
      previewSmallCoords: undefined,
    });
  }

  setLargeWinner() {
    const smallWinnerBoard: MediumBoard = [];
    for (let i = 0; i < 3; i++) {
      const smallRow: SmallBoard[] = [];
      for (let j = 0; j < 3; j++) {
        const boxWinnerBoard: SmallBoard = [];
        for (let k = 0; k < 3; k++) {
          const boxRow: Winner[] = [];
          for (let l = 0; l < 3; l++) {
            boxRow.push(this.checkSmallWinner(this.state.board[i][j][k][l]));
          }
          boxWinnerBoard.push(boxRow);
        }
        smallRow.push(boxWinnerBoard);
      }
      smallWinnerBoard.push(smallRow);
    }

    const winnerBoard: SmallBoard = [];
    for (let i = 0; i < 3; i++) {
      const row: Winner[] = [];
      for (let j = 0; j < 3; j++) {
        row.push(this.checkSmallWinner(smallWinnerBoard[i][j]));
      }
      winnerBoard.push(row);
    }

    const winner = this.checkSmallWinner(winnerBoard);

    if (winner !== undefined) {
      this.setState({
        board: genBoard("large", winner) as LargeBoard,
      });
    }
  }

  setMediumWinner(mediumProps: MediumProps) {
    const coords = mediumProps.coords;
    const winnerBoard: SmallBoard = [];
    for (let i = 0; i < 3; i++) {
      const row: Winner[] = [];
      for (let j = 0; j < 3; j++) {
        row.push(this.checkSmallWinner(mediumProps.board[i][j]));
      }
      winnerBoard.push(row);
    }
    const winner = this.checkSmallWinner(winnerBoard);

    if (winner !== undefined) {
      const board = this.state.board;
      board[coords[1]][coords[0]] = genBoard("medium", winner) as MediumBoard;
      this.setState({
        board: board,
      });
    }
  }

  setSmallWinner(mediumProps: MediumProps, smallProps: SmallProps) {
    const coords = mediumProps.coords;
    const smallCoords = smallProps.coords;
    const winner = this.checkSmallWinner(smallProps.board);
    if (winner !== undefined) {
      const board = this.state.board;
      board[coords[1]][coords[0]][smallCoords[1]][smallCoords[0]] = genBoard(
        "small",
        winner
      ) as SmallBoard;
      this.setState({
        board: board,
      });
    }
  }

  checkSmallWinner(board: SmallBoard): Winner {
    if (board.flat().filter((x) => x === undefined).length === 9) {
      return undefined;
    }
    for (let i = 0; i < 3; i++) {
      if (
        board[i][0] !== undefined &&
        board[i][0] === board[i][1] &&
        board[i][0] === board[i][2]
      ) {
        return board[i][0];
      }
    }
    for (let i = 0; i < 3; i++) {
      if (
        board[0][i] !== undefined &&
        board[0][i] === board[1][i] &&
        board[0][i] === board[2][i]
      ) {
        return board[0][i];
      }
    }
    if (
      board[0][0] !== undefined &&
      board[0][0] === board[1][1] &&
      board[0][0] === board[2][2]
    ) {
      return board[0][0];
    }
    if (
      board[0][2] !== undefined &&
      board[0][2] === board[1][1] &&
      board[0][2] === board[2][0]
    ) {
      return board[0][2];
    }
    if (board.flat().filter((x) => x === undefined).length === 0) {
      return null;
    }
    return undefined;
  }

  renderMedium(coords: [number, number]) {
    return (
      <MediumTTT
        previewCoords={
          this.state.previewCoords !== undefined
            ? this.state.previewCoords[0] === coords[0] &&
              this.state.previewCoords[1] === coords[1]
              ? this.state.previewSmallCoords
              : undefined
            : undefined
        }
        onPreviewTurn={this.previewTurn}
        onUnpreviewTurn={this.unpreviewTurn}
        onFinishTurn={this.finishTurn}
        coords={coords}
        turn={this.state.turn}
        winner={undefined}
        active={
          this.state.activeCoords[0] === coords[0] &&
          this.state.activeCoords[1] === coords[1]
        }
        board={this.state.board[coords[1]][coords[0]]}
        activeCoords={this.state.activeSmallCoords}
      />
    );
  }

  render() {
    return (
      <div className="flex flex-col items-center justify-center w-screen h-screen">
        <div className="flex flex-col gap-4 p-8 transition-shadow bg-gray-100 shadow-md rounded-xl">
          <div className="flex flex-row gap-4">
            {this.renderMedium([0, 0])}
            {this.renderMedium([1, 0])}
            {this.renderMedium([2, 0])}
          </div>
          <div className="flex flex-row gap-4">
            {this.renderMedium([0, 1])}
            {this.renderMedium([1, 1])}
            {this.renderMedium([2, 1])}
          </div>
          <div className="flex flex-row gap-4">
            {this.renderMedium([0, 2])}
            {this.renderMedium([1, 2])}
            {this.renderMedium([2, 2])}
          </div>
        </div>
      </div>
    );
  }
}

export default LargeTTT;
