import optuna
import subprocess
import joblib
import statistics

n_cores = -1
n_files = 120


def calc_score_each(seed: int, tabu_tenure: int, ratio: float):
    in_file = open(f"tools/in/{seed:04}.txt", 'r')
    # out_file = open(f"tools/out/{seed:04}.txt", 'w')
    process = subprocess.run(["cargo", "run", "--release", str(tabu_tenure), str(ratio)],
                             stdin=in_file, stdout=subprocess.DEVNULL, encoding='utf-8', stderr=subprocess.PIPE)
    return int(process.stderr.split(':')[-1].strip())


def calc_score(tabu_tenure: int, ratio: float):
    return joblib.Parallel(n_jobs=n_cores)(
        joblib.delayed(calc_score_each)(i, tabu_tenure, ratio) for i in range(n_files)
    )


def objective(trial: optuna.trial.Trial):
    tabu_tenure = trial.suggest_int("tabu_tenure", 2, 200)
    ratio = trial.suggest_float("t1", 0.0001, 0.9999)
    scores = calc_score(tabu_tenure, ratio)
    return statistics.mean(scores)


if __name__ == "__main__":
    study = optuna.create_study(direction="maximize",
                                storage="sqlite:///ahc014.db",
                                study_name="tune_fixratio_tabu_tenure")
    study.optimize(objective, n_trials=200)
    print(study.best_params)
    print(study.best_value)
