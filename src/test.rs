#[cfg(test)]
mod tests {
    use crate::ContractError;
    use crate::contract::{execute, instantiate, query};
    use crate::msg::{ExecuteMsg, GetCustomResponse, InstantiateMsg, QueryMsg};

    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary, Attribute, DepsMut, Env, MessageInfo, Response};

    const CREATER_ADDR: &str = "creater";

    //Quick init of the contract
    fn mock_initialization(
        deps: DepsMut,
        info: Option<MessageInfo>,
        msg: Option<InstantiateMsg>,
    ) -> (Response, MessageInfo, Env) {
        let env = mock_env();
        let info = match info {
            Some(info) => info,
            None => mock_info(CREATER_ADDR, &coins(1000, "earth")),
        };

        let msg = match msg {
            Some(msg) => msg,
            None => InstantiateMsg {
                owner: info.sender.to_string(),
            },
        };

        let res = instantiate(deps, env.clone(), info.clone(), msg).unwrap();

        (res, info, env)
    }

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let (res, info, _) = mock_initialization(deps.as_mut(), None, None);

        assert_eq!(
            res.attributes,
            vec![
                Attribute {
                    key: "method".to_string(),
                    value: "instantiate".to_string()
                },
                Attribute {
                    key: "owner".to_string(),
                    value: info.sender.to_string()
                },
            ]
        );
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies();

        let (_res, info, env) = mock_initialization(deps.as_mut(), None, None);

        //Do custom execute
        let msg = ExecuteMsg::Custom {};
        let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        //Do custom execute expect error
        let err_info = mock_info("error_addr", &[]);
        let msg = ExecuteMsg::Custom {};
        let err = execute(deps.as_mut(), env, err_info, msg);

        match err {
            Err(ContractError::Unauthorized{}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        //Do custom query
        let msg = QueryMsg::GetCustom {};
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let value: GetCustomResponse = from_binary(&res).unwrap();

        assert_eq!(CREATER_ADDR, value.owner);
    }
}
