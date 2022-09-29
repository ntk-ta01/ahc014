import optuna
import subprocess
import joblib
import statistics

n_cores = -1
n_files = 300


def calc_score_each(seed: int, t0: float, t1: float):
    in_file = open(f"tools/in/{seed:04}.txt", 'r')
    # out_file = open(f"tools/out/{seed:04}.txt", 'w')
    process = subprocess.run(["cargo", "run", "--release", str(t0), str(t1)],
                             stdin=in_file, stdout=subprocess.DEVNULL, encoding='utf-8', stderr=subprocess.PIPE)
    return int(process.stderr.split(':')[-1].strip())


def calc_score(t0: float, t1: float):
    return joblib.Parallel(n_jobs=n_cores)(
        joblib.delayed(calc_score_each)(i, t0, t1) for i in range(n_files)
    )


def objective(trial: optuna.trial.Trial):
    t0 = trial.suggest_float("t0", 5000.0, 8000.0)
    t1 = trial.suggest_float("t1", 2999.9, t0)
    # insert_tabu_tenure = trial.suggest_int("insert_tabu_tenure", 2, 80)
    # remove_tabu_tenure = trial.suggest_int("remove_tabu_tenure", 2, 160)
    # ratio_r = trial.suggest_float("ratio_r", 0.0001, 0.9999)
    # ratio_l = trial.suggest_float("ratio_l", 0.0001, ratio_r)
    scores = calc_score(t0, t1)
    return statistics.mean(scores)


if __name__ == "__main__":
    study = optuna.create_study(direction="maximize",
                                storage="sqlite:///ahc014.db",
                                study_name="tune_temp2")
    study.optimize(objective, n_trials=300)
    print(study.best_params)
    print(study.best_value)
