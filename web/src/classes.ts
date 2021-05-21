export class UserData {
  programs: [Program, ...Array<Program>];
  settings: UserSettings;

  constructor(programs: [Program, ...Array<Program>], settings: UserSettings) {
    this.programs = programs;
    this.settings = settings;
  }

  static from(json: UserDataObject) {
    return new UserData(
      [
        Program.from(json.programs[0]),
        ...json.programs
          .slice(1)
          .map((program: ProgramObject) => Program.from(program)),
      ],
      UserSettings.from(json.settings)
    );
  }
}

export type UserDataObject = {
  programs: [ProgramObject, ...Array<ProgramObject>];
  settings: {};
};

export class UserSettings {
  static from(json: UserSettingsObject) {
    return new UserSettings();
  }
}

export type UserSettingsObject = {};

export class Program {
  institution: string;
  courses: Array<Course>;
  settings: ProgramSettings;

  constructor(
    institution: string,
    courses: Array<Course>,
    settings: ProgramSettings
  ) {
    this.institution = institution;
    this.courses = courses;
    this.settings = settings;
  }

  static from(json: ProgramObject) {
    return new Program(
      json.institution,
      json.courses.map((course) => Course.from(course)),
      ProgramSettings.from(json.settings)
    );
  }

  predicted(): number | null {
    return (
      this.courses.reduce(
        (sumSoFar, course) => sumSoFar + course.predicted() * course.credits,
        0
      ) /
      this.courses.reduce((sumSoFar, course) => sumSoFar + course.credits, 0)
    );
  }

  completion(): number {
    return (
      this.courses.reduce(
        (sumSoFar, course) => sumSoFar + course.completion() * course.credits,
        0
      ) /
      this.courses.reduce((sumSoFar, course) => sumSoFar + course.credits, 0)
    );
  }
}

export type ProgramObject = {
  institution: string;
  courses: Array<CourseObject>;
  settings: ProgramSettingsObject;
};

export class ProgramSettings {
  static from(json: ProgramSettingsObject) {
    return new ProgramSettings();
  }
}

export type ProgramSettingsObject = {};

export class Course {
  name: string;
  code: string;
  credits: number;
  rootGrade: WeightedAverageGrade;
  finished: boolean;
  finalGrade: PercentGrade;

  constructor(
    name: string,
    code: string,
    rootGrade: WeightedAverageGrade,
    finished: boolean,
    finalGrade: PercentGrade | null
  ) {
    this.name = name;
    this.code = code;
    this.rootGrade = rootGrade;
    this.finished = finished;
    this.finalGrade = finalGrade;
  }

  static from(json: CourseObject) {
    return new Course(
      json.name,
      json.code,
      // @ts-ignore
      Grade.from(json.rootGrade),
      json.finished,
      Grade.from(json.finalGrade)
    );
  }

  predicted(): number | null {
    return this.rootGrade.predicted();
  }

  completion(): number {
    return this.rootGrade.completion();
  }
}

export type CourseObject = {
  name: string;
  code: string;
  credits: number;
  rootGrade: GradeObject;
  finished: boolean;
  finalGrade: GradeObject;
};

export class Grade {
  name: string;
  predicted: () => number | null;
  completion: () => number;
  type:
    | "FractionGrade"
    | "WeightedAverageGrade"
    | "PercentGrade"
    | "AverageGrade";

  constructor(name: string) {
    this.name = name;
    // @ts-ignore
    this.type = this.constructor.name;
  }

  static from(json: GradeObject) {
    if (json.type === "FractionGrade") {
      // @ts-ignore
      return new FractionGrade(json.name, json.numerator, json.denominator);
    } else if (json.type === "WeightedAverageGrade") {
      return new WeightedAverageGrade(
        json.name,
        // @ts-ignore
        json.grades.map((grade: GradeObject) => Grade.from(grade)),
        // @ts-ignore
        json.weights
      );
    } else if (json.type === "PercentGrade") {
      // @ts-ignore
      return new PercentGrade(json.name, json.percent);
    } else {
      // if (json.type === "AverageGrade")
      return new AverageGrade(
        json.name,
        // @ts-ignore
        json.grades.map((grade: GradeObject) => Grade.from(grade))
      );
    }
  }
}

export type GradeObject = {
  name: string;
  type:
    | "FractionGrade"
    | "WeightedAverageGrade"
    | "PercentGrade"
    | "AverageGrade";
};

export class FractionGrade extends Grade {
  numerator: number;
  denominator: number;

  constructor(name: string, numerator: number, denominator: number) {
    super(name);
    this.numerator = numerator;
    this.denominator = denominator;
  }

  predicted = (): number | null => {
    if (this.denominator) {
      return this.numerator / this.denominator;
    }
  };

  completion = (): number => {
    return this.numerator && this.denominator ? 1 : 0;
  };
}

export interface FractionGradeObject extends GradeObject {
  numerator: number;
  denominator: number;
}

export class PercentGrade extends Grade {
  percent: number | null;

  constructor(name: string, percent: number | null) {
    super(name);
    this.percent = percent;
  }

  predicted = (): number => {
    return this.percent / 100;
  };

  completion = (): number => {
    return this.percent === 0 || this.percent ? 1 : 0;
  };
}

export interface PercentGradeObject extends GradeObject {
  percent: number | null;
}

export class AverageGrade extends Grade {
  grades: Array<Grade>;

  constructor(name: string, grades: Array<Grade>) {
    super(name);
    this.grades = grades;
  }

  predicted = (): number | null => {
    let completedGrades = this.grades.filter((grade) => grade);
    if (completedGrades) {
      return (
        completedGrades.reduce(
          (sumSoFar, grade) => sumSoFar + grade.predicted(),
          0
        ) / completedGrades.length
      );
    }
  };

  completion = (): number => {
    return this.grades.reduce(
      (sumSoFar, grade) => sumSoFar + grade.completion(),
      0
    );
  };
}

export interface AverageGradeObject extends GradeObject {
  grades: Array<GradeObject>;
}

export class WeightedAverageGrade extends Grade {
  grades: Array<Grade>;
  weights: Array<number>;

  constructor(name: string, grades: Array<Grade>, weights: Array<number>) {
    super(name);
    this.grades = grades;
    this.weights = weights;
  }

  predicted = (): number | null => {
    let sumSoFar = 0;
    for (let i in this.grades) {
      sumSoFar +=
        this.weights[i] *
        this.grades[i].predicted() *
        this.grades[i].completion();
    }
    return sumSoFar / this.completion();
  };

  completion = (): number => {
    let sumSoFar = 0;
    for (let i in this.grades) {
      sumSoFar += this.weights[i] * this.grades[i].completion();
    }
    return sumSoFar;
  };
}

export interface WeightedAverageObject extends GradeObject {
  grades: Array<GradeObject>;
  weights: Array<number>;
}
