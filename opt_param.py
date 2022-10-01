import optuna
import subprocess
import joblib
import statistics

n_cores = -1
n_files = 300


def calc_score_each(seed: int, select_prob: float):
    in_file = open(f"tools/in/{seed:04}.txt", 'r')
    # out_file = open(f"tools/out/{seed:04}.txt", 'w')
    process = subprocess.run(["cargo", "run", "--release", str(select_prob)],
                             stdin=in_file, stdout=subprocess.DEVNULL, encoding='utf-8', stderr=subprocess.PIPE)
    return int(process.stderr.split(':')[-1].strip())


def calc_score(select_prob: float):
    return joblib.Parallel(n_jobs=n_cores)(
        joblib.delayed(calc_score_each)(i, select_prob) for i in range(n_files)
    )


def objective(trial: optuna.trial.Trial):
    # t0 = trial.suggest_float("t0", 5000.0, 8000.0)
    # t1 = trial.suggest_float("t1", 2999.9, t0)
    select_prob = trial.suggest_float("select_prob", 0.01, 0.99)
    scores = calc_score(select_prob)
    return statistics.mean(scores)


if __name__ == "__main__":
    study = optuna.create_study(direction="maximize",
                                storage="sqlite:///ahc014.db",
                                study_name="tune_select_prob")
    study.optimize(objective, n_trials=150)
    print(study.best_params)
    print(study.best_value)
