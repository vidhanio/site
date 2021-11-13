import type { NextPage } from "next";
import Head from "next/head";
import Image from "next/image";
import React from "react";

type winnerValue = "X" | "O" | null | undefined;
type smallBoard = winnerValue[][];
type MediumBoard = smallBoard[][];

interface TTTProps {
  coords: [number, number];
  turn: "X" | "O";
  winner: winnerValue;
}

interface BoxProps extends TTTProps {
  onFinishTurn: (boxProps: BoxProps) => void;
}

interface SmallTTTProps extends TTTProps {
  board: smallBoard;
  onFinishTurn: (smallProps: SmallTTTProps, boxProps: BoxProps) => void;
}

interface MediumTTTState {
  board: MediumBoard;
  winner: winnerValue;
  turn: "X" | "O";
}

class Box extends React.Component<BoxProps> {
  constructor(props: BoxProps) {
    super(props);
    this.handleFinishTurn = this.handleFinishTurn.bind(this);
  }

  handleFinishTurn() {
    this.props.onFinishTurn(this.props);
  }

  render() {
    return (
      <button
        className={`font-mono text-xl transition-colors ${
          this.props.winner !== undefined ? "text-red-400" : "text-blue-400"
        }`}
        onClick={
          this.props.winner === undefined
            ? this.handleFinishTurn
            : () => undefined
        }
      >
        {this.props.winner === undefined
          ? "."
          : this.props.winner === null
          ? "T"
          : this.props.winner}
      </button>
    );
  }
}

class SmallTTT extends React.Component<SmallTTTProps> {
  constructor(props: SmallTTTProps) {
    super(props);
    this.state = {
      board: [
        [undefined, undefined, undefined],
        [undefined, undefined, undefined],
        [undefined, undefined, undefined],
      ],
      winner: undefined,
      turn: "X",
    };

    this.finishTurn = this.finishTurn.bind(this);
  }

  finishTurn(boxProps: BoxProps) {
    const newBoard = this.props.board;
    newBoard[boxProps.coords[1]][boxProps.coords[0]] = this.props.turn;
    this.props.onFinishTurn(this.props, boxProps);
  }

  renderBox(coords: [number, number]) {
    return (
      <Box
        onFinishTurn={this.finishTurn}
        coords={coords}
        turn={this.props.turn}
        winner={this.props.board[coords[1]][coords[0]]}
      />
    );
  }

  render() {
    return (
      <div className="flex flex-col">
        <div className="flex flex-row">
          {this.renderBox([0, 0])}
          {this.renderBox([1, 0])}
          {this.renderBox([2, 0])}
        </div>
        <div className="flex flex-row">
          {this.renderBox([0, 1])}
          {this.renderBox([1, 1])}
          {this.renderBox([2, 1])}
        </div>
        <div className="flex flex-row">
          {this.renderBox([0, 2])}
          {this.renderBox([1, 2])}
          {this.renderBox([2, 2])}
        </div>
      </div>
    );
  }
}

class MediumTicTacToe extends React.Component<{}, MediumTTTState> {
  constructor(props: {}) {
    super(props);

    this.state = {
      board: [
        [
          [
            [undefined, undefined, undefined],
            [undefined, undefined, undefined],
            [undefined, undefined, undefined],
          ],
          [
            [undefined, undefined, undefined],
            [undefined, undefined, undefined],
            [undefined, undefined, undefined],
          ],
          [
            [undefined, undefined, undefined],
            [undefined, undefined, undefined],
            [undefined, undefined, undefined],
          ],
        ],
        [
          [
            [undefined, undefined, undefined],
            [undefined, undefined, undefined],
            [undefined, undefined, undefined],
          ],
          [
            [undefined, undefined, undefined],
            [undefined, undefined, undefined],
            [undefined, undefined, undefined],
          ],
          [
            [undefined, undefined, undefined],
            [undefined, undefined, undefined],
            [undefined, undefined, undefined],
          ],
        ],
        [
          [
            [undefined, undefined, undefined],
            [undefined, undefined, undefined],
            [undefined, undefined, undefined],
          ],
          [
            [undefined, undefined, undefined],
            [undefined, undefined, undefined],
            [undefined, undefined, undefined],
          ],
          [
            [undefined, undefined, undefined],
            [undefined, undefined, undefined],
            [undefined, undefined, undefined],
          ],
        ],
      ],
      winner: undefined,
      turn: "X",
    };

    this.finishTurn = this.finishTurn.bind(this);
  }

  finishTurn(smallProps: SmallTTTProps, boxProps: BoxProps) {
    const board = this.state.board;
    board[smallProps.coords[1]][smallProps.coords[0]][boxProps.coords[1]][
      boxProps.coords[0]
    ] = smallProps.turn;
    this.setState({ board: board });

    this.checkSmallWinner(smallProps);

    this.setState({ turn: this.state.turn === "X" ? "O" : "X" });
  }

  setSmallWinner(smallProps: SmallTTTProps, winner: winnerValue) {
    const coords = smallProps.coords;
    if (winner !== undefined) {
      const board = this.state.board;
      board[coords[1]][coords[0]] = [
        [winner, winner, winner],
        [winner, winner, winner],
        [winner, winner, winner],
      ];
      this.setState({
        board: board,
      });
    }
  }

  checkSmallWinner(smallProps: SmallTTTProps) {
    const coords = smallProps.coords;
    const board = this.state.board[coords[1]][coords[0]];
    for (let i = 0; i < 3; i++) {
      if (
        board[i][0] !== undefined &&
        board[i][0] === board[i][1] &&
        board[i][0] === board[i][2]
      ) {
        this.setSmallWinner(smallProps, board[i][0]);
        return;
      }
    }
    for (let i = 0; i < 3; i++) {
      if (
        board[0][i] !== undefined &&
        board[0][i] === board[1][i] &&
        board[0][i] === board[2][i]
      ) {
        this.setSmallWinner(smallProps, board[0][i]);
        return;
      }
    }
    if (
      board[0][0] !== undefined &&
      board[0][0] === board[1][1] &&
      board[0][0] === board[2][2]
    ) {
      this.setSmallWinner(smallProps, board[0][0]);
      return;
    }
    if (
      board[0][2] !== undefined &&
      board[0][2] === board[1][1] &&
      board[0][2] === board[2][0]
    ) {
      this.setSmallWinner(smallProps, board[0][2]);
      return;
    }
    if (board.flat().filter((x) => x === undefined).length === 0) {
      this.setSmallWinner(smallProps, null);
    }
  }

  renderSmallTTT(coords: [number, number]) {
    return (
      <SmallTTT
        onFinishTurn={this.finishTurn}
        coords={coords}
        turn={this.state.turn}
        winner={undefined}
        board={this.state.board[coords[1]][coords[0]]}
      />
    );
  }

  render() {
    return (
      <div className="flex flex-col gap-2">
        <div className="flex flex-row gap-2">
          {this.renderSmallTTT([0, 0])}
          {this.renderSmallTTT([1, 0])}
          {this.renderSmallTTT([2, 0])}
        </div>
        <div className="flex flex-row gap-2">
          {this.renderSmallTTT([0, 1])}
          {this.renderSmallTTT([1, 1])}
          {this.renderSmallTTT([2, 1])}
        </div>
        <div className="flex flex-row gap-2">
          {this.renderSmallTTT([0, 2])}
          {this.renderSmallTTT([1, 2])}
          {this.renderSmallTTT([2, 2])}
        </div>
      </div>
    );
  }
}

export default MediumTicTacToe;
