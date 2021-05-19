export class UserData {
  programs: [Program, ...Array<Program>];
  settings: UserSettings;

  constructor(programs: [Program, ...Array<Program>], settings: UserSettings) {
    this.programs = programs;
    this.settings = settings;
  }
}

export class UserSettings {}

export class Program {
  institution: string;
  courses: Array<Course>;
  settings: ProgramSettings;

  constructor(institution: string, courses: Array<Course>) {
    this.institution = institution;
    this.courses = courses;
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

export class ProgramSettings {}

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

  predicted(): number | null {
    return this.rootGrade.predicted();
  }

  completion(): number {
    return this.rootGrade.completion();
  }
}

export class Grade {
  name: string;
  predicted: () => number | null;
  completion: () => number;
  type: string;

  constructor(name: string) {
    this.name = name;
    this.type = this.constructor.name;
  }
}

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
